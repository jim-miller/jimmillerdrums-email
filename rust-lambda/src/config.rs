use std::env;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub email_bucket: String,
    pub incoming_prefix: String,
    pub forward_to_email: String,
    pub max_email_size_mb: u32,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let email_bucket = env::var("EMAIL_BUCKET")
            .map_err(|_| ConfigError::MissingEnvVar("EMAIL_BUCKET".to_string()))?;

        let incoming_prefix = env::var("INCOMING_PREFIX")
            .map_err(|_| ConfigError::MissingEnvVar("INCOMING_PREFIX".to_string()))?;

        let forward_to_email = env::var("FORWARD_TO_EMAIL")
            .map_err(|_| ConfigError::MissingEnvVar("FORWARD_TO_EMAIL".to_string()))?;

        let max_email_size_mb = env::var("MAX_EMAIL_SIZE_MB")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(10);

        if !(1..=10).contains(&max_email_size_mb) {
            return Err(ConfigError::InvalidValue(format!(
                "MAX_EMAIL_SIZE_MB must be between 1 and 10, got {}",
                max_email_size_mb
            )));
        }

        Ok(Config {
            email_bucket,
            incoming_prefix,
            forward_to_email,
            max_email_size_mb,
        })
    }

    pub fn new(email_bucket: String, incoming_prefix: String, forward_to_email: String) -> Self {
        Config {
            email_bucket,
            incoming_prefix,
            forward_to_email,
            max_email_size_mb: 10,
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
        assert_eq!(config.max_email_size_mb, 10);
    }

    #[test]
    fn test_config_from_env_missing_vars() {
        // Ensure environment variables are not set
        env::remove_var("EMAIL_BUCKET");
        env::remove_var("INCOMING_PREFIX");
        env::remove_var("FORWARD_TO_EMAIL");
        env::remove_var("MAX_EMAIL_SIZE_MB");

        let result = Config::from_env();
        assert!(result.is_err());

        if let Err(ConfigError::MissingEnvVar(var_name)) = result {
            assert_eq!(var_name, "EMAIL_BUCKET");
        } else {
            panic!("Expected MissingEnvVar error");
        }
    }

    #[test]
    fn test_config_max_email_size_default() {
        env::set_var("EMAIL_BUCKET", "test-bucket");
        env::set_var("INCOMING_PREFIX", "incoming");
        env::set_var("FORWARD_TO_EMAIL", "test@example.com");
        env::remove_var("MAX_EMAIL_SIZE_MB");

        let result = Config::from_env();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().max_email_size_mb, 10);

        env::remove_var("EMAIL_BUCKET");
        env::remove_var("INCOMING_PREFIX");
        env::remove_var("FORWARD_TO_EMAIL");
    }

    #[test]
    fn test_config_max_email_size_custom() {
        env::set_var("EMAIL_BUCKET", "test-bucket");
        env::set_var("INCOMING_PREFIX", "incoming");
        env::set_var("FORWARD_TO_EMAIL", "test@example.com");
        env::set_var("MAX_EMAIL_SIZE_MB", "5");

        let result = Config::from_env();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().max_email_size_mb, 5);

        env::remove_var("EMAIL_BUCKET");
        env::remove_var("INCOMING_PREFIX");
        env::remove_var("FORWARD_TO_EMAIL");
        env::remove_var("MAX_EMAIL_SIZE_MB");
    }

    #[test]
    fn test_config_max_email_size_invalid() {
        env::set_var("EMAIL_BUCKET", "test-bucket");
        env::set_var("INCOMING_PREFIX", "incoming");
        env::set_var("FORWARD_TO_EMAIL", "test@example.com");
        env::set_var("MAX_EMAIL_SIZE_MB", "15");

        let result = Config::from_env();
        assert!(result.is_err());

        env::remove_var("EMAIL_BUCKET");
        env::remove_var("INCOMING_PREFIX");
        env::remove_var("FORWARD_TO_EMAIL");
        env::remove_var("MAX_EMAIL_SIZE_MB");
    }
}
