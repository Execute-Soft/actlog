use thiserror::Error;
#[allow(dead_code, unused_imports, unused_variables)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Cloud provider error: {0}")]
    CloudProviderError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Invalid date format: {0}")]
    DateParseError(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    #[error("Dry run mode - no changes made")]
    DryRunMode,

    #[error("User cancelled operation")]
    UserCancelled,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("AWS SDK error: {0}")]
    AwsError(String),

    #[error("GCP error: {0}")]
    GcpError(String),

    #[error("Azure error: {0}")]
    AzureError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

impl From<aws_sdk_ec2::Error> for AppError {
    fn from(err: aws_sdk_ec2::Error) -> Self {
        AppError::AwsError(err.to_string())
    }
}

impl From<aws_sdk_s3::Error> for AppError {
    fn from(err: aws_sdk_s3::Error) -> Self {
        AppError::AwsError(err.to_string())
    }
}

impl From<aws_sdk_cloudwatch::Error> for AppError {
    fn from(err: aws_sdk_cloudwatch::Error) -> Self {
        AppError::AwsError(err.to_string())
    }
}

impl From<aws_sdk_costexplorer::Error> for AppError {
    fn from(err: aws_sdk_costexplorer::Error) -> Self {
        AppError::AwsError(err.to_string())
    }
}

impl From<aws_sdk_autoscaling::Error> for AppError {
    fn from(err: aws_sdk_autoscaling::Error) -> Self {
        AppError::AwsError(err.to_string())
    }
}
