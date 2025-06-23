use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Operation failed: {0}")]
    Operation(String),

    #[error("Unknown error occurred")]
    Unknown,
}

#[allow(dead_code)]
impl AppError {
    pub fn config<S: Into<String>>(message: S) -> Self {
        AppError::Config(message.into())
    }

    pub fn validation<S: Into<String>>(message: S) -> Self {
        AppError::Validation(message.into())
    }

    pub fn operation<S: Into<String>>(message: S) -> Self {
        AppError::Operation(message.into())
    }
}
