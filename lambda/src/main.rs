use anyhow::{Context, Result};
use aws_config::load_from_env;
use aws_lambda_events::event::ses::SimpleEmailEvent;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_ses::Client as SesClient;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use regex::Regex;
use std::env;
use tracing::info;

/// Domain-specific types using the Newtype pattern
#[derive(Debug, Clone)]
struct MessageId(String);

#[derive(Debug, Clone)]
struct EmailAddress(String);

#[derive(Debug, Clone)]
struct BucketName(String);

/// Email processor with AWS clients initialized in global scope
struct EmailProcessor {
    s3_client: S3Client,
    ses_client: SesClient,
    email_bucket: BucketName,
    forward_to_email: EmailAddress,
}

impl EmailProcessor {
    /// Initialize the email processor with AWS clients
    async fn new() -> Result<Self> {
        let config = load_from_env().await;
        
        let email_bucket = env::var("EMAIL_BUCKET")
            .context("EMAIL_BUCKET environment variable not set")
            .map(BucketName)?;
            
        let forward_to_email = env::var("FORWARD_TO_EMAIL")
            .context("FORWARD_TO_EMAIL environment variable not set")
            .map(EmailAddress)?;

        Ok(Self {
            s3_client: S3Client::new(&config),
            ses_client: SesClient::new(&config),
            email_bucket,
            forward_to_email,
        })
    }

    /// Process the SES event and route emails accordingly
    async fn process_event(&self, event: SimpleEmailEvent) -> Result<()> {
        info!("Processing SES event with {} records", event.records.len());

        for record in event.records {
            let message_id = record.ses.mail.message_id.clone();
            self.process_email_record(record).await
                .with_context(|| format!("Failed to process email record: {:?}", message_id))?;
        }

        Ok(())
    }

    /// Process a single email record
    async fn process_email_record(&self, record: aws_lambda_events::event::ses::SimpleEmailRecord) -> Result<()> {
        let message_id = MessageId(
            record.ses.mail.message_id
                .context("Missing message_id in SES record")?
        );
        let recipients: Vec<EmailAddress> = record.ses.receipt.recipients
            .into_iter()
            .map(EmailAddress)
            .collect();

        info!("Processing email {} for {} recipients", message_id.0, recipients.len());

        let email_content = self.get_email_from_s3(&message_id).await?;

        for recipient in recipients {
            self.route_email(&recipient, &email_content).await
                .with_context(|| format!("Failed to route email to {}", recipient.0))?;
        }

        Ok(())
    }

    /// Retrieve email content from S3
    async fn get_email_from_s3(&self, message_id: &MessageId) -> Result<String> {
        let response = self
            .s3_client
            .get_object()
            .bucket(&self.email_bucket.0)
            .key(&message_id.0)
            .send()
            .await
            .context("Failed to get email from S3")?;

        let body = response
            .body
            .collect()
            .await
            .context("Failed to read S3 object body")?;

        String::from_utf8(body.into_bytes().to_vec())
            .context("Failed to convert email content to UTF-8")
    }

    /// Route email based on recipient address
    async fn route_email(&self, recipient: &EmailAddress, email_content: &str) -> Result<()> {
        let recipient_local = self.extract_local_part(recipient);

        if self.should_forward_to_gmail(&recipient_local) {
            info!("Forwarding {} to Gmail: {}", recipient.0, self.forward_to_email.0);
            self.forward_email_to_gmail(recipient, email_content, None).await
        } else {
            info!("Handling {} separately (future: WorkMail/IMAP)", recipient.0);
            // For now, forward with [INFO] prefix for separate handling
            self.forward_email_to_gmail(recipient, email_content, Some("[INFO] ")).await
        }
    }

    /// Extract local part of email address (part before @)
    fn extract_local_part(&self, email: &EmailAddress) -> String {
        email.0
            .split('@')
            .next()
            .unwrap_or("")
            .to_lowercase()
    }

    /// Determine if email should be forwarded to Gmail
    fn should_forward_to_gmail(&self, recipient_local: &str) -> bool {
        matches!(recipient_local, "booking" | "contact" | "hello" | "support")
    }

    /// Forward email to Gmail with optional subject prefix
    async fn forward_email_to_gmail(
        &self,
        original_recipient: &EmailAddress,
        email_content: &str,
        subject_prefix: Option<&str>,
    ) -> Result<()> {
        let modified_email = self.modify_email_headers(
            email_content,
            original_recipient,
            &self.forward_to_email,
            subject_prefix,
        )?;

        self.ses_client
            .send_raw_email()
            .raw_message(
                aws_sdk_ses::types::RawMessage::builder()
                    .data(aws_sdk_ses::primitives::Blob::new(modified_email.into_bytes()))
                    .build()
            )
            .send()
            .await
            .context("Failed to send email via SES")?;

        info!(
            "Successfully forwarded email from {} to {}",
            original_recipient.0, self.forward_to_email.0
        );

        Ok(())
    }

    /// Modify email headers for forwarding
    fn modify_email_headers(
        &self,
        email_content: &str,
        original_recipient: &EmailAddress,
        forward_to: &EmailAddress,
        subject_prefix: Option<&str>,
    ) -> Result<String> {
        let mut modified_content = email_content.to_string();

        // Replace To header
        let to_regex = Regex::new(r"(?m)^To:.*$")?;
        modified_content = to_regex
            .replace(&modified_content, format!("To: {}", forward_to.0))
            .to_string();

        // Extract and handle From header for Reply-To
        let from_regex = Regex::new(r"(?m)^From:(.*)$")?;
        if let Some(from_match) = from_regex.find(email_content) {
            let original_from = from_match.as_str().trim_start_matches("From:").trim();

            // Add Reply-To if it doesn't exist
            let reply_to_regex = Regex::new(r"(?m)^Reply-To:")?;
            if !reply_to_regex.is_match(&modified_content) {
                modified_content = from_regex
                    .replace(&modified_content, format!("$0\nReply-To: {original_from}"))
                    .to_string();
            }

            // Replace From header with verified domain
            let clean_from = original_from.replace(['<', '>'], "");
            modified_content = from_regex
                .replace(
                    &modified_content,
                    format!("From: {clean_from} via {} <{}>", original_recipient.0, original_recipient.0),
                )
                .to_string();
        }

        // Add subject prefix if specified
        if let Some(prefix) = subject_prefix {
            let subject_regex = Regex::new(r"(?m)^Subject:(.*)$")?;
            modified_content = subject_regex
                .replace(&modified_content, format!("Subject: {prefix}$1"))
                .to_string();
        }

        // Add forwarding information headers
        let subject_regex = Regex::new(r"(?m)^Subject:(.*)$")?;
        modified_content = subject_regex
            .replace(
                &modified_content,
                format!("$0\nX-Forwarded-For: {}\nX-Original-To: {}", original_recipient.0, original_recipient.0),
            )
            .to_string();

        Ok(modified_content)
    }
}

/// Lambda function handler
async fn function_handler(event: LambdaEvent<SimpleEmailEvent>) -> Result<(), Error> {
    let processor = EmailProcessor::new().await?;
    processor.process_event(event.payload).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize tracing for CloudWatch compatibility
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
