use thiserror::Error;

pub type Internal = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Error, Debug)]
#[error("Failed in {service} external service: {message}")]
pub struct ExternalServiceError {
    pub service: String,
    pub message: String,
}

#[derive(Error, Debug)]
#[error("Invalid argument '{name}' with value '{value}': {reason}")]
pub struct InvalidArgument {
    pub name: String,
    pub value: String,
    pub reason: String,
}
