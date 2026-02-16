use mailparse::parse_mail;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MimeError {
    #[error("Failed to parse email: {0}")]
    ParseError(#[from] mailparse::MailParseError),
    #[error("Invalid email structure: {0}")]
    InvalidStructure(String),
}

/// Modify email headers while preserving the entire MIME body
/// Replaces From, To, and Reply-To headers with new values
/// Strips DKIM-Signature and internal SES headers to prevent duplication errors
pub fn modify_email_headers(
    raw_email: &[u8],
    new_from: &str,
    new_to: &str,
    new_reply_to: &str,
) -> Result<Vec<u8>, MimeError> {
    let parsed = parse_mail(raw_email)?;

    // Find where headers end and body begins
    let header_end = find_header_body_boundary(raw_email).ok_or_else(|| {
        MimeError::InvalidStructure("Could not find header/body boundary".to_string())
    })?;

    // Build new headers
    let mut new_headers = Vec::new();

    // Copy headers EXCEPT those we are replacing or those that cause conflicts
    for header in &parsed.headers {
        let key = header.get_key();

        if is_forbidden_header(&key) {
            continue;
        }

        new_headers.push(format!("{}: {}\r\n", key, header.get_value()));
    }

    // Add new headers
    new_headers.push(format!("From: {}\r\n", new_from));
    new_headers.push(format!("To: {}\r\n", new_to));
    new_headers.push(format!("Reply-To: {}\r\n", new_reply_to));

    // Combine new headers with original body
    let mut result = Vec::new();
    for header in new_headers {
        result.extend_from_slice(header.as_bytes());
    }
    // Add blank line between headers and body
    result.extend_from_slice(b"\r\n");
    result.extend_from_slice(&raw_email[header_end..]);

    Ok(result)
}

/// Helper to identify headers that must be removed to avoid SES conflicts
fn is_forbidden_header(key: &str) -> bool {
    let k = key.to_lowercase();
    matches!(
        k.as_str(),
        "from"
            | "to"
            | "reply-to"
            | "dkim-signature"
            | "return-path"
            | "sender"
            | "message-id"
            | "x-ses-message-id"
            | "x-ses-outgoing"
    )
}

/// Find the boundary between headers and body (double CRLF)
fn find_header_body_boundary(email: &[u8]) -> Option<usize> {
    for i in 0..email.len().saturating_sub(3) {
        if email[i] == b'\r'
            && email[i + 1] == b'\n'
            && email[i + 2] == b'\r'
            && email[i + 3] == b'\n'
        {
            return Some(i + 4); // Return position AFTER the double CRLF
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_simple_email() {
        let email = b"From: old@example.com\r\nTo: oldrecipient@example.com\r\nSubject: Test\r\n\r\nBody content";
        let result = modify_email_headers(
            email,
            "new@example.com",
            "newrecipient@example.com",
            "reply@example.com",
        );

        assert!(result.is_ok());
        let modified = result.unwrap();
        let modified_str = String::from_utf8_lossy(&modified);

        assert!(modified_str.contains("From: new@example.com"));
        assert!(modified_str.contains("To: newrecipient@example.com"));
        assert!(modified_str.contains("Reply-To: reply@example.com"));
        assert!(modified_str.contains("Subject: Test"));
        assert!(modified_str.contains("Body content"));
    }

    #[test]
    fn test_strips_dkim_signature() {
        let email = b"From: old@example.com\r\nDKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed;\r\nSubject: Test\r\n\r\nBody";
        let result = modify_email_headers(
            email,
            "new@example.com",
            "newrecipient@example.com",
            "reply@example.com",
        );

        assert!(result.is_ok());
        let modified = result.unwrap();
        let modified_str = String::from_utf8_lossy(&modified);

        // Ensure DKIM-Signature is GONE
        assert!(
            !modified_str.contains("DKIM-Signature"),
            "DKIM-Signature should be removed"
        );
        assert!(modified_str.contains("Subject: Test"));
    }

    #[test]
    fn test_modify_multipart_email() {
        let email = b"From: old@example.com\r\nTo: oldrecipient@example.com\r\nContent-Type: multipart/alternative; boundary=\"test\"\r\n\r\n--test\r\nContent-Type: text/plain\r\n\r\nPlain text\r\n--test\r\nContent-Type: text/html\r\n\r\n<html>HTML</html>\r\n--test--";
        let result = modify_email_headers(
            email,
            "new@example.com",
            "newrecipient@example.com",
            "reply@example.com",
        );

        assert!(result.is_ok());
        let modified = result.unwrap();
        let modified_str = String::from_utf8_lossy(&modified);

        assert!(modified_str.contains("Content-Type: multipart/alternative; boundary=\"test\""));
        assert!(modified_str.contains("--test\r\nContent-Type: text/plain"));
        assert!(modified_str.contains("Plain text"));
    }

    #[test]
    fn test_find_boundary() {
        let email = b"H: V\r\n\r\nBody";

        let boundary = find_header_body_boundary(email);
        assert_eq!(boundary, Some(8));
    }
}
