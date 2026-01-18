use crate::domain::{EmailAddress, EmailBody, MessageId, S3Key, Subject};
use crate::email::{extract_sender_name, parse_email, EmailError};
use aws_sdk_s3::Client as S3Client;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
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

pub struct ForwardEmailRequest {
    pub bucket: String,
    pub incoming_path: String,
    pub message_id: MessageId,
    pub original_from: String,
    pub forward_to: EmailAddress,
}

pub async fn forward_email(
    context: &AppContext,
    request: ForwardEmailRequest,
) -> Result<String, AwsError> {
    let s3_key = S3Key::try_from(format!("{}/{}", request.incoming_path, request.message_id))?;

    let email_bytes = retrieve_email_from_s3(&context.s3_client, &request.bucket, &s3_key).await?;

    let parsed = parse_email(&email_bytes)?;

    let sender_name = extract_sender_name(&request.original_from);
    let from_address = format!(
        "\"{}\" (via jimmillerdrums.com) <forwarder@jimmillerdrums.com>",
        sender_name
    );

    let message_id = send_email_via_ses(
        &context.ses_client,
        &from_address,
        &request.forward_to,
        &parsed.from,
        &parsed.subject,
        &parsed.body,
    )
    .await?;

    Ok(message_id)
}
