use std::fmt;

use azalea_auth::AuthError as AzaleaAuthError;
use claude_sdk::Error as ClaudeError;
use config::ConfigError;
use std::io::Error as IOError;
use tokio::task::JoinError as TokioJoinError;

#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Join(TokioJoinError),
    Auth(AzaleaAuthError),
    Claude(ClaudeError),
    IO(IOError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "Config error: {e}"),
            AppError::Join(e) => write!(f, "Config error: {e}"),
            AppError::Auth(e) => write!(f, "Auth error: {e}"),
            AppError::Claude(e) => write!(f, "Auth error: {e}"),
            AppError::IO(e) => write!(f, "Auth error: {e}"),
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
            AppError::Claude(e) => Some(e),
            AppError::IO(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(e)
    }
}

impl From<TokioJoinError> for AppError {
    fn from(e: TokioJoinError) -> Self {
        AppError::Join(e)
    }
}

impl From<AzaleaAuthError> for AppError {
    fn from(e: AzaleaAuthError) -> Self {
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

impl From<ClaudeError> for AppError {
    fn from(e: ClaudeError) -> Self {
        AppError::Claude(e)
    }
}

impl From<IOError> for AppError {
    fn from(e: IOError) -> Self {
        AppError::IO(e)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
