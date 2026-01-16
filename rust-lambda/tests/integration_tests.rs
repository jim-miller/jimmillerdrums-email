use email_processor::{
    extract_sender_name, parse_email, EmailAddress, EmailBody, MessageId, S3Key, Subject,
};

const TEST_EMAIL: &[u8] = b"From: John Doe <john@example.com>\r\n\
Subject: Test Email Subject\r\n\
Content-Type: text/plain; charset=utf-8\r\n\
\r\n\
This is a test email body with some content.";

const MULTIPART_EMAIL: &[u8] = b"From: Jane Smith <jane@example.com>\r\n\
Subject: Multipart Test\r\n\
Content-Type: multipart/alternative; boundary=\"boundary123\"\r\n\
\r\n\
--boundary123\r\n\
Content-Type: text/plain; charset=utf-8\r\n\
\r\n\
Plain text version of the email.\r\n\
--boundary123\r\n\
Content-Type: text/html; charset=utf-8\r\n\
\r\n\
<html><body>HTML version</body></html>\r\n\
--boundary123--";

#[test]
fn test_parse_simple_email_integration() {
    let result = parse_email(TEST_EMAIL);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert_eq!(parsed.subject.as_str(), "Test Email Subject");
    assert_eq!(parsed.from.as_str(), "john@example.com");
    assert!(parsed.body.as_str().contains("test email body"));
}

#[test]
fn test_parse_multipart_email() {
    let result = parse_email(MULTIPART_EMAIL);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert_eq!(parsed.subject.as_str(), "Multipart Test");
    assert_eq!(parsed.from.as_str(), "jane@example.com");
}

#[test]
fn test_extract_sender_name_with_quotes() {
    let name = extract_sender_name("\"John Doe\" <john@example.com>");
    assert_eq!(name, "John Doe");
}

#[test]
fn test_extract_sender_name_without_quotes() {
    let name = extract_sender_name("John Doe <john@example.com>");
    assert_eq!(name, "John Doe");
}

#[test]
fn test_extract_sender_name_email_only() {
    let name = extract_sender_name("john@example.com");
    assert_eq!(name, "john");
}

#[test]
fn test_domain_types_validation() {
    // Valid email
    let email = EmailAddress::try_from("test@example.com".to_string());
    assert!(email.is_ok());

    // Invalid email
    let invalid = EmailAddress::try_from("not-an-email".to_string());
    assert!(invalid.is_err());

    // Valid message ID
    let msg_id = MessageId::try_from("msg-123".to_string());
    assert!(msg_id.is_ok());

    // Invalid message ID (empty)
    let invalid_id = MessageId::try_from("".to_string());
    assert!(invalid_id.is_err());

    // S3 Key
    let key = S3Key::try_from("incoming/test.eml".to_string());
    assert!(key.is_ok());

    // Subject and Body (always valid)
    let subject = Subject::try_from("Test Subject".to_string());
    assert!(subject.is_ok());

    let body = EmailBody::try_from("Test body".to_string());
    assert!(body.is_ok());
}

#[test]
fn test_email_parsing_edge_cases() {
    // Email with no subject
    let no_subject = b"From: test@example.com\r\n\r\nBody only";
    let result = parse_email(no_subject);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().subject.as_str(), "Forwarded Email");

    // Email with special characters in subject
    let special_chars =
        b"From: test@example.com\r\nSubject: Test: Special [chars] & symbols!\r\n\r\nBody";
    let result = parse_email(special_chars);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().subject.as_str(),
        "Test: Special [chars] & symbols!"
    );
}

#[test]
fn test_quoted_printable_handling() {
    let qp_email = b"From: test@example.com\r\n\
Subject: Test\r\n\
Content-Type: text/plain; charset=utf-8\r\n\
Content-Transfer-Encoding: quoted-printable\r\n\
\r\n\
This is a test with special chars: =C3=A9 =C3=A0";

    let result = parse_email(qp_email);
    assert!(result.is_ok());
    // mailparse should handle quoted-printable decoding
}
