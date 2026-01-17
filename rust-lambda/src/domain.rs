use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid email address: {0}")]
    InvalidEmail(String),
    #[error("Invalid message ID: {0}")]
    InvalidMessageId(String),
    #[error("Invalid S3 key: {0}")]
    InvalidS3Key(String),
    #[error("Invalid subject: {0}")]
    InvalidSubject(String),
    #[error("Invalid email body: {0}")]
    InvalidEmailBody(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EmailAddress {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains('@') && value.len() > 3 {
            Ok(EmailAddress(value))
        } else {
            Err(DomainError::InvalidEmail(value))
        }
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageId(String);

impl MessageId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for MessageId {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.is_empty() {
            Ok(MessageId(value))
        } else {
            Err(DomainError::InvalidMessageId(value))
        }
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct S3Key(String);

impl S3Key {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for S3Key {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.is_empty() {
            Ok(S3Key(value))
        } else {
            Err(DomainError::InvalidS3Key(value))
        }
    }
}

impl fmt::Display for S3Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Subject(String);

impl Subject {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Subject {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Subject(value))
    }
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct EmailBody(String);

impl EmailBody {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EmailBody {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(EmailBody(value))
    }
}

impl fmt::Display for EmailBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct SesEvent {
    #[serde(rename = "Records")]
    pub records: Vec<SesRecord>,
}

#[derive(Debug, Deserialize)]
pub struct SesRecord {
    pub ses: SesMessage,
}

#[derive(Debug, Deserialize)]
pub struct SesMessage {
    pub mail: SesMail,
}

#[derive(Debug, Deserialize)]
pub struct SesMail {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub source: String,
    pub destination: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_address_valid() {
        let email = EmailAddress::try_from("test@example.com".to_string());
        assert!(email.is_ok());
    }

    #[test]
    fn test_email_address_invalid() {
        let email = EmailAddress::try_from("invalid".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn test_message_id_valid() {
        let msg_id = MessageId::try_from("abc123".to_string());
        assert!(msg_id.is_ok());
    }

    #[test]
    fn test_message_id_invalid() {
        let msg_id = MessageId::try_from("".to_string());
        assert!(msg_id.is_err());
    }
}
