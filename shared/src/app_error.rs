use std::fmt;

use azalea_auth::AuthError;
use config::ConfigError;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Join(JoinError),
    Auth(AuthError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "Config error: {e}"),
            AppError::Join(e) => write!(f, "Config error: {e}"),
            AppError::Auth(e) => write!(f, "Auth error: {e}"),
            AppError::Custom(s) => write!(f, "Error: {s}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Config(e) => Some(e),
            AppError::Join(e) => Some(e),
            AppError::Auth(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(e)
    }
}

impl From<JoinError> for AppError {
    fn from(e: JoinError) -> Self {
        AppError::Join(e)
    }
}

impl From<AuthError> for AppError {
    fn from(e: AuthError) -> Self {
        AppError::Auth(e)
    }
}

impl From<Box<dyn std::any::Any + Send>> for AppError {
    fn from(e: Box<dyn std::any::Any + Send>) -> Self {
        AppError::Custom(format!("{e:?}"))
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::Custom(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
