use std::env;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub email_bucket: String,
    pub incoming_prefix: String,
    pub forward_to_email: String,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let email_bucket = env::var("EMAIL_BUCKET")
            .map_err(|_| ConfigError::MissingEnvVar("EMAIL_BUCKET".to_string()))?;

        let incoming_prefix = env::var("INCOMING_PREFIX")
            .map_err(|_| ConfigError::MissingEnvVar("INCOMING_PREFIX".to_string()))?;

        let forward_to_email = env::var("FORWARD_TO_EMAIL")
            .map_err(|_| ConfigError::MissingEnvVar("FORWARD_TO_EMAIL".to_string()))?;

        Ok(Config {
            email_bucket,
            incoming_prefix,
            forward_to_email,
        })
    }

    pub fn new(email_bucket: String, incoming_prefix: String, forward_to_email: String) -> Self {
        Config {
            email_bucket,
            incoming_prefix,
            forward_to_email,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_new() {
        let config = Config::new(
            "test-bucket".to_string(),
            "incoming".to_string(),
            "test@example.com".to_string(),
        );

        assert_eq!(config.email_bucket, "test-bucket");
        assert_eq!(config.incoming_prefix, "incoming");
        assert_eq!(config.forward_to_email, "test@example.com");
    }

    #[test]
    fn test_config_from_env_missing_vars() {
        // Ensure environment variables are not set
        env::remove_var("EMAIL_BUCKET");
        env::remove_var("INCOMING_PREFIX");
        env::remove_var("FORWARD_TO_EMAIL");

        let result = Config::from_env();
        assert!(result.is_err());

        if let Err(ConfigError::MissingEnvVar(var_name)) = result {
            assert_eq!(var_name, "EMAIL_BUCKET");
        } else {
            panic!("Expected MissingEnvVar error");
        }
    }
}
