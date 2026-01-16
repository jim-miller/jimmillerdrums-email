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
}
