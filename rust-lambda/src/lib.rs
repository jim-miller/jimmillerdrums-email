#![forbid(unsafe_code)]

pub mod aws;
pub mod config;
pub mod domain;
pub mod email;

pub use aws::*;
pub use domain::*;
pub use email::*;

use serde_json::{json, Value};
use tracing::{error, info};

pub async fn process_ses_event(
    event: SesEvent,
    context: &AppContext,
    config: &config::Config,
) -> Result<Value, lambda_runtime::Error> {
    info!("Processing SES event with {} records", event.records.len());

    let record = event
        .records
        .first()
        .ok_or_else(|| lambda_runtime::Error::from("No records in SES event"))?;

    let message_id = MessageId::try_from(record.ses.mail.message_id.clone())?;
    let original_from = record.ses.mail.source.clone();
    let destination = record
        .ses
        .mail
        .destination
        .first()
        .ok_or_else(|| lambda_runtime::Error::from("No destination in SES event"))?;

    info!(
        "Processing email: {} from {} to {}",
        message_id, original_from, destination
    );

    let forward_to = EmailAddress::try_from(config.forward_to_email.clone())?;

    let request = ForwardEmailRequest {
        bucket: config.email_bucket.clone(),
        incoming_path: config.incoming_prefix.clone(),
        message_id,
        original_from,
        forward_to,
    };

    match forward_email(context, request).await {
        Ok(forwarded_message_id) => {
            info!("Email forwarded successfully: {}", forwarded_message_id);
            Ok(json!({
                "statusCode": 200,
                "body": json!({
                    "message": "Email forwarded successfully",
                    "forwardedMessageId": forwarded_message_id
                }).to_string()
            }))
        }
        Err(e) => {
            error!("Error forwarding email: {}", e);
            Err(lambda_runtime::Error::from(e.to_string()))
        }
    }
}
