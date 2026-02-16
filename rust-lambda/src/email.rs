use crate::domain::{EmailAddress, EmailBody, Subject};
use mailparse::parse_mail;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Failed to parse email: {0}")]
    ParseError(#[from] mailparse::MailParseError),
    #[error("Missing header: {0}")]
    MissingHeader(String),
    #[error("Domain error: {0}")]
    DomainError(#[from] crate::domain::DomainError),
}

pub struct ParsedEmail {
    pub subject: Subject,
    pub from: EmailAddress,
    pub body: EmailBody,
}

pub fn parse_email(raw_email: &[u8]) -> Result<ParsedEmail, EmailError> {
    let parsed = parse_mail(raw_email)?;

    let subject = parsed
        .headers
        .iter()
        .find(|h| h.get_key().eq_ignore_ascii_case("Subject"))
        .map(|h| h.get_value())
        .unwrap_or_else(|| "Forwarded Email".to_string());

    let from_header = parsed
        .headers
        .iter()
        .find(|h| h.get_key().eq_ignore_ascii_case("From"))
        .map(|h| h.get_value())
        .ok_or_else(|| EmailError::MissingHeader("From".to_string()))?;

    let from = extract_email_address(&from_header)?;

    let body_text = parsed
        .get_body()
        .unwrap_or_else(|_| String::from_utf8_lossy(parsed.raw_bytes).to_string());

    Ok(ParsedEmail {
        subject: Subject::try_from(subject)?,
        from: EmailAddress::try_from(from)?,
        body: EmailBody::try_from(body_text.trim().to_string())?,
    })
}

fn extract_email_address(from_header: &str) -> Result<String, EmailError> {
    if let Some(start) = from_header.find('<') {
        if let Some(end) = from_header.find('>') {
            return Ok(from_header[start + 1..end].to_string());
        }
    }

    let parts: Vec<&str> = from_header.split_whitespace().collect();
    for part in parts {
        if part.contains('@') {
            return Ok(part.to_string());
        }
    }

    Ok(from_header.to_string())
}

pub fn extract_sender_name(from_header: &str) -> String {
    if let Some(start) = from_header.find('<') {
        let name = from_header[..start].trim().trim_matches('"');
        if !name.is_empty() {
            return name.to_string();
        }
    }

    if let Some(email_addr) = from_header.split('@').next() {
        return email_addr.to_string();
    }

    from_header.to_string()
}

/// Extract Reply-To information from raw email, falling back to From header
/// Returns (email_address, display_name)
pub fn extract_reply_to_info(raw_email: &[u8]) -> Result<(String, String), EmailError> {
    let parsed = parse_mail(raw_email)?;

    // Try Reply-To header first
    if let Some(reply_to_header) = parsed
        .headers
        .iter()
        .find(|h| h.get_key().eq_ignore_ascii_case("Reply-To"))
    {
        let reply_to_value = reply_to_header.get_value();
        let email = extract_email_address(&reply_to_value)?;
        let name = extract_sender_name(&reply_to_value);
        return Ok((email, name));
    }

    // Fall back to From header
    if let Some(from_header) = parsed
        .headers
        .iter()
        .find(|h| h.get_key().eq_ignore_ascii_case("From"))
    {
        let from_value = from_header.get_value();
        let email = extract_email_address(&from_value)?;
        let name = extract_sender_name(&from_value);
        return Ok((email, name));
    }

    Err(EmailError::MissingHeader("From or Reply-To".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_email() {
        let email = b"From: test@example.com\r\nSubject: Test\r\n\r\nBody content";
        let parsed = parse_email(email);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_extract_email_from_angle_brackets() {
        let result = extract_email_address("John Doe <john@example.com>");
        assert_eq!(result.unwrap(), "john@example.com");
    }

    #[test]
    fn test_extract_sender_name() {
        let name = extract_sender_name("\"John Doe\" <john@example.com>");
        assert_eq!(name, "John Doe");
    }

    #[test]
    fn test_extract_reply_to_with_reply_to_header() {
        let email = b"From: sender@example.com\r\nReply-To: Test Org <replyto@example.com>\r\nSubject: Test\r\n\r\nBody";
        let result = extract_reply_to_info(email);
        assert!(result.is_ok());
        let (email_addr, name) = result.unwrap();
        assert_eq!(email_addr, "replyto@example.com");
        assert_eq!(name, "Test Org");
    }

    #[test]
    fn test_extract_reply_to_fallback_to_from() {
        let email = b"From: Test Sender <sender@example.com>\r\nSubject: Test\r\n\r\nBody";
        let result = extract_reply_to_info(email);
        assert!(result.is_ok());
        let (email_addr, name) = result.unwrap();
        assert_eq!(email_addr, "sender@example.com");
        assert_eq!(name, "Test Sender");
    }

    #[test]
    fn test_extract_reply_to_missing_headers() {
        let email = b"Subject: Test\r\n\r\nBody";
        let result = extract_reply_to_info(email);
        assert!(result.is_err());
    }
}
