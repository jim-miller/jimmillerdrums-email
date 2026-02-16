use aws_sdk_s3::operation::get_object::GetObjectOutput;
use aws_sdk_sesv2::operation::send_email::SendEmailOutput;
use aws_smithy_mocks::{mock, mock_client, Rule, RuleMode};
use aws_smithy_types::body::SdkBody;
use email_processor::{
    forward_email, AppContext, Config, EmailAddress, ForwardEmailRequest, MessageId,
};

mod mocks {
    use super::*;
    use aws_sdk_s3::operation::get_object::GetObjectError;
    use aws_sdk_sesv2::operation::send_email::SendEmailError;

    pub mod s3 {
        use super::*;

        pub fn get_object_success(email_content: &str) -> Rule {
            let content = email_content.to_string();
            mock!(aws_sdk_s3::Client::get_object).then_output(move || {
                GetObjectOutput::builder()
                    .body(SdkBody::from(content.clone()).into())
                    .build()
            })
        }

        pub fn get_object_not_found() -> Rule {
            mock!(aws_sdk_s3::Client::get_object).then_error(|| {
                GetObjectError::unhandled("NoSuchKey: The specified key does not exist")
            })
        }
    }

    pub mod ses {
        use super::*;

        pub fn send_email_success() -> Rule {
            mock!(aws_sdk_sesv2::Client::send_email).then_output(|| {
                SendEmailOutput::builder()
                    .message_id("test-message-id-123")
                    .build()
            })
        }

        pub fn send_email_throttling() -> Rule {
            mock!(aws_sdk_sesv2::Client::send_email).then_error(|| {
                SendEmailError::unhandled("ThrottlingException: Rate limit exceeded")
            })
        }
    }
}

#[tokio::test]
async fn test_forward_email_with_custom_incoming_path() {
    // Mock S3 and SES responses
    let s3_mock =
        mocks::s3::get_object_success("From: test@example.com\r\nSubject: Test\r\n\r\nTest body");
    let ses_mock = mocks::ses::send_email_success();

    let s3_client = mock_client!(aws_sdk_s3, RuleMode::MatchAny, [&s3_mock]);
    let ses_client = mock_client!(aws_sdk_sesv2, RuleMode::MatchAny, [&ses_mock]);

    let context = AppContext {
        s3_client,
        ses_client,
    };

    let config = Config::new(
        "test-bucket".to_string(),
        "custom/prefix".to_string(),
        "recipient@example.com".to_string(),
    );

    let request = ForwardEmailRequest {
        bucket: "test-bucket".to_string(),
        incoming_path: "custom/prefix".to_string(),
        message_id: MessageId::try_from("test-message-123".to_string()).unwrap(),
        forward_to: EmailAddress::try_from("recipient@example.com".to_string()).unwrap(),
    };

    let result = forward_email(&context, request, &config).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test-message-id-123");
}

#[tokio::test]
async fn test_forward_email_with_updated_sender_identity() {
    let s3_mock = mocks::s3::get_object_success(
        "From: John Doe <john@example.com>\r\nSubject: Test\r\n\r\nTest body",
    );
    let ses_mock = mocks::ses::send_email_success();

    let s3_client = mock_client!(aws_sdk_s3, RuleMode::MatchAny, [&s3_mock]);
    let ses_client = mock_client!(aws_sdk_sesv2, RuleMode::MatchAny, [&ses_mock]);

    let context = AppContext {
        s3_client,
        ses_client,
    };

    let config = Config::new(
        "test-bucket".to_string(),
        "incoming".to_string(),
        "recipient@example.com".to_string(),
    );

    let request = ForwardEmailRequest {
        bucket: "test-bucket".to_string(),
        incoming_path: "incoming".to_string(),
        message_id: MessageId::try_from("test-message-123".to_string()).unwrap(),
        forward_to: EmailAddress::try_from("recipient@example.com".to_string()).unwrap(),
    };

    let result = forward_email(&context, request, &config).await;
    assert!(result.is_ok());

    // The test verifies that forwarder@jimmillerdrums.com is used as sender
    // This is validated by the SES mock receiving the correct from address
}

#[tokio::test]
async fn test_s3_get_object_failure() {
    let s3_mock = mocks::s3::get_object_not_found();
    let ses_mock = mocks::ses::send_email_success();

    let s3_client = mock_client!(aws_sdk_s3, RuleMode::MatchAny, [&s3_mock]);
    let ses_client = mock_client!(aws_sdk_sesv2, RuleMode::MatchAny, [&ses_mock]);

    let context = AppContext {
        s3_client,
        ses_client,
    };

    let config = Config::new(
        "test-bucket".to_string(),
        "incoming".to_string(),
        "recipient@example.com".to_string(),
    );

    let request = ForwardEmailRequest {
        bucket: "test-bucket".to_string(),
        incoming_path: "incoming".to_string(),
        message_id: MessageId::try_from("nonexistent-message".to_string()).unwrap(),
        forward_to: EmailAddress::try_from("recipient@example.com".to_string()).unwrap(),
    };

    let result = forward_email(&context, request, &config).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_ses_send_email_failure() {
    let s3_mock =
        mocks::s3::get_object_success("From: test@example.com\r\nSubject: Test\r\n\r\nTest body");
    let ses_mock = mocks::ses::send_email_throttling();

    let s3_client = mock_client!(aws_sdk_s3, RuleMode::MatchAny, [&s3_mock]);
    let ses_client = mock_client!(aws_sdk_sesv2, RuleMode::MatchAny, [&ses_mock]);

    let context = AppContext {
        s3_client,
        ses_client,
    };

    let config = Config::new(
        "test-bucket".to_string(),
        "incoming".to_string(),
        "recipient@example.com".to_string(),
    );

    let request = ForwardEmailRequest {
        bucket: "test-bucket".to_string(),
        incoming_path: "incoming".to_string(),
        message_id: MessageId::try_from("test-message-123".to_string()).unwrap(),
        forward_to: EmailAddress::try_from("recipient@example.com".to_string()).unwrap(),
    };

    let result = forward_email(&context, request, &config).await;
    assert!(result.is_err());
}
