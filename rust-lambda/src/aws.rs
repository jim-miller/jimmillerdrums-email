use crate::domain::{EmailAddress, EmailBody, MessageId, S3Key, Subject};
use crate::email::EmailError;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_sesv2::primitives::Blob;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message, RawMessage};
use aws_sdk_sesv2::Client as SesClient;
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum AwsError {
    #[error("S3 error: {0}")]
    S3Error(String),
    #[error("SES error: {0}")]
    SesError(String),
    #[error("Email parsing error: {0}")]
    EmailError(#[from] EmailError),
    #[error("MIME error: {0}")]
    MimeError(#[from] crate::mime::MimeError),
    #[error("Domain error: {0}")]
    DomainError(#[from] crate::domain::DomainError),
}

pub struct AppContext {
    pub s3_client: S3Client,
    pub ses_client: SesClient,
}

impl AppContext {
    pub fn new(config: &aws_config::SdkConfig) -> Self {
        Self {
            s3_client: S3Client::new(config),
            ses_client: SesClient::new(config),
        }
    }
}

pub async fn retrieve_email_from_s3(
    client: &S3Client,
    bucket: &str,
    key: &S3Key,
) -> Result<Vec<u8>, AwsError> {
    info!("Retrieving email from S3: {}/{}", bucket, key);

    let response = client
        .get_object()
        .bucket(bucket)
        .key(key.as_str())
        .send()
        .await
        .map_err(|e| AwsError::S3Error(e.to_string()))?;

    let bytes = response
        .body
        .collect()
        .await
        .map_err(|e| AwsError::S3Error(e.to_string()))?
        .into_bytes()
        .to_vec();

    info!("Retrieved {} bytes from S3", bytes.len());
    Ok(bytes)
}

pub async fn send_email_via_ses(
    client: &SesClient,
    from: &str,
    to: &EmailAddress,
    reply_to: &EmailAddress,
    subject: &Subject,
    body: &EmailBody,
) -> Result<String, AwsError> {
    info!("Sending email via SES to {}", to);

    let destination = Destination::builder().to_addresses(to.as_str()).build();

    let subject_content = Content::builder()
        .data(subject.as_str())
        .build()
        .map_err(|e| AwsError::SesError(e.to_string()))?;

    let body_content = Content::builder()
        .data(body.as_str())
        .build()
        .map_err(|e| AwsError::SesError(e.to_string()))?;

    let message = Message::builder()
        .subject(subject_content)
        .body(Body::builder().text(body_content).build())
        .build();

    let email_content = EmailContent::builder().simple(message).build();

    let response = client
        .send_email()
        .from_email_address(from)
        .destination(destination)
        .reply_to_addresses(reply_to.as_str())
        .content(email_content)
        .send()
        .await
        .map_err(|e| AwsError::SesError(e.to_string()))?;

    let message_id = response.message_id().unwrap_or("unknown");
    info!("Email sent successfully: {}", message_id);
    Ok(message_id.to_string())
}

/// Send raw MIME email via SES (supports multipart, attachments, etc.)
pub async fn send_raw_email_via_ses(
    client: &SesClient,
    raw_email: &[u8],
    from_email: &str,
) -> Result<String, AwsError> {
    info!("Sending raw email via SES ({} bytes)", raw_email.len());

    let raw_message = RawMessage::builder()
        .data(Blob::new(raw_email))
        .build()
        .map_err(|e| AwsError::SesError(format!("Failed to build raw message: {}", e)))?;

    let email_content = EmailContent::builder().raw(raw_message).build();

    let response = client
        .send_email()
        .from_email_address(from_email)
        .content(email_content)
        .send()
        .await
        .map_err(|e| {
            AwsError::SesError(format!("SES send_email failed: {} (details: {:?})", e, e))
        })?;

    let message_id = response.message_id().unwrap_or("unknown");
    info!("Raw email sent successfully: {}", message_id);
    Ok(message_id.to_string())
}

pub fn validate_email_size(email_bytes: &[u8], max_size_mb: u32) -> Result<(), AwsError> {
    let size_bytes = email_bytes.len();
    let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
    let max_size_bytes = (max_size_mb as usize) * 1024 * 1024;

    if size_bytes > max_size_bytes {
        return Err(AwsError::SesError(format!(
            "Email size ({:.2} MB) exceeds maximum allowed size ({} MB)",
            size_mb, max_size_mb
        )));
    }

    info!(
        "Email size validation passed: {:.2} MB / {} MB",
        size_mb, max_size_mb
    );
    Ok(())
}

pub struct ForwardEmailRequest {
    pub bucket: String,
    pub incoming_path: String,
    pub message_id: MessageId,
    pub forward_to: EmailAddress,
}

pub async fn forward_email(
    context: &AppContext,
    request: ForwardEmailRequest,
    config: &crate::config::Config,
) -> Result<String, AwsError> {
    let s3_key = S3Key::try_from(format!("{}/{}", request.incoming_path, request.message_id))?;

    let email_bytes = retrieve_email_from_s3(&context.s3_client, &request.bucket, &s3_key).await?;

    validate_email_size(&email_bytes, config.max_email_size_mb)?;

    let (reply_to_email, sender_name) = crate::email::extract_reply_to_info(&email_bytes)?;

    let from_display_address = format!(
        "\"{}\" (via jimmillerdrums.com) <forwarder@jimmillerdrums.com>",
        sender_name
    );

    let modified_email = crate::mime::modify_email_headers(
        &email_bytes,
        &from_display_address,
        request.forward_to.as_str(),
        &reply_to_email,
    )?;

    let message_id = send_raw_email_via_ses(
        &context.ses_client,
        &modified_email,
        "forwarder@jimmillerdrums.com",
    )
    .await?;

    Ok(message_id)
}
