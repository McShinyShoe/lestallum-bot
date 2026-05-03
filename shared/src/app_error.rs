use std::fmt;

use config::ConfigError;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Join(JoinError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "Config error: {e}"),
            AppError::Join(e) => write!(f, "Config error: {e}"),
            AppError::Custom(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Config(e) => Some(e),
            AppError::Join(e) => Some(e),
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

pub type Result<T> = std::result::Result<T, AppError>;
