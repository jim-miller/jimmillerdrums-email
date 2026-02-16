#![forbid(unsafe_code)]

pub mod aws;
pub mod config;
pub mod domain;
pub mod email;
pub mod mime;

pub use aws::*;
pub use config::Config;
pub use domain::*;
pub use email::*;
pub use mime::*;

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
    let destination = record
        .ses
        .mail
        .destination
        .first()
        .ok_or_else(|| lambda_runtime::Error::from("No destination in SES event"))?;

    info!("Processing email: {} to {}", message_id, destination);

    if is_report_email(destination) {
        info!("Skipping forwarding for report email to: {}", destination);
        return Ok(json!({
            "statusCode": 200,
            "body": json!({
                "message": "Report email processed but not forwarded",
                "messageId": message_id
            }).to_string()
        }));
    }

    let forward_to = EmailAddress::try_from(config.forward_to_email.clone())?;

    let request = ForwardEmailRequest {
        bucket: config.email_bucket.clone(),
        incoming_path: config.incoming_prefix.clone(),
        message_id,
        forward_to,
    };

    match forward_email(context, request, config).await {
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

fn is_report_email(destination: &str) -> bool {
    destination.starts_with("dmarc@") || destination.starts_with("reports@")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_report_email_dmarc() {
        assert!(is_report_email("dmarc@jimmillerdrums.com"));
    }

    #[test]
    fn test_is_report_email_reports() {
        assert!(is_report_email("reports@jimmillerdrums.com"));
    }

    #[test]
    fn test_is_not_report_email() {
        assert!(!is_report_email("info@jimmillerdrums.com"));
        assert!(!is_report_email("contact@jimmillerdrums.com"));
        assert!(!is_report_email("hello@jimmillerdrums.com"));
    }
}
