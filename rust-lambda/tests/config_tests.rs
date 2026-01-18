use aws_sdk_s3::operation::get_object::GetObjectOutput;
use aws_sdk_sesv2::operation::send_email::SendEmailOutput;
use aws_smithy_mocks::{mock, mock_client, RuleMode};
use aws_smithy_types::body::SdkBody;
use email_processor::config::Config;
use email_processor::{process_ses_event, AppContext, SesEvent, SesMail, SesMessage, SesRecord};

#[tokio::test]
async fn test_config_based_processing() {
    // Create test configuration without environment variables
    let config = Config::new(
        "test-bucket".to_string(),
        "custom/incoming".to_string(),
        "test@example.com".to_string(),
    );

    // Mock AWS responses
    let s3_mock = mock!(aws_sdk_s3::Client::get_object).then_output(|| {
        GetObjectOutput::builder()
            .body(
                SdkBody::from("From: sender@example.com\r\nSubject: Test\r\n\r\nTest body").into(),
            )
            .build()
    });

    let ses_mock = mock!(aws_sdk_sesv2::Client::send_email).then_output(|| {
        SendEmailOutput::builder()
            .message_id("test-message-id")
            .build()
    });

    let s3_client = mock_client!(aws_sdk_s3, RuleMode::MatchAny, [&s3_mock]);
    let ses_client = mock_client!(aws_sdk_sesv2, RuleMode::MatchAny, [&ses_mock]);

    let context = AppContext {
        s3_client,
        ses_client,
    };

    // Create test SES event
    let ses_event = SesEvent {
        records: vec![SesRecord {
            ses: SesMessage {
                mail: SesMail {
                    message_id: "test-message-123".to_string(),
                    source: "sender@example.com".to_string(),
                    destination: vec!["recipient@jimmillerdrums.com".to_string()],
                },
            },
        }],
    };

    let result = process_ses_event(ses_event, &context, &config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_custom_incoming_prefix_in_config() {
    let config = Config::new(
        "test-bucket".to_string(),
        "reports/dmarc".to_string(),
        "test@example.com".to_string(),
    );

    // Mock S3 to verify the correct path is used
    let s3_mock = mock!(aws_sdk_s3::Client::get_object)
        .match_requests(|req| req.key().unwrap().starts_with("reports/dmarc/"))
        .then_output(|| {
            GetObjectOutput::builder()
                .body(
                    SdkBody::from(
                        "From: sender@example.com\r\nSubject: DMARC Report\r\n\r\nReport body",
                    )
                    .into(),
                )
                .build()
        });

    let ses_mock = mock!(aws_sdk_sesv2::Client::send_email).then_output(|| {
        SendEmailOutput::builder()
            .message_id("test-message-id")
            .build()
    });

    let s3_client = mock_client!(aws_sdk_s3, RuleMode::MatchAny, [&s3_mock]);
    let ses_client = mock_client!(aws_sdk_sesv2, RuleMode::MatchAny, [&ses_mock]);

    let context = AppContext {
        s3_client,
        ses_client,
    };

    let ses_event = SesEvent {
        records: vec![SesRecord {
            ses: SesMessage {
                mail: SesMail {
                    message_id: "dmarc-report-123".to_string(),
                    source: "noreply-dmarc-support@google.com".to_string(),
                    destination: vec!["dmarc@jimmillerdrums.com".to_string()],
                },
            },
        }],
    };

    let result = process_ses_event(ses_event, &context, &config).await;
    assert!(result.is_ok());
}
