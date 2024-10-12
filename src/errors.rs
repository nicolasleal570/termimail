use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GenerateEmailError {
    Request(reqwest::Error),
    Io(std::io::Error),
    Serde(serde_json::Error),
    ApiError(String), // For API-specific errors
    ExistingAccount(String),
}

impl fmt::Display for GenerateEmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateEmailError::Request(e) => write!(f, "Request error: {}", e),
            GenerateEmailError::Io(e) => write!(f, "I/O error: {}", e),
            GenerateEmailError::Serde(e) => write!(f, "Serialization error: {}", e),
            GenerateEmailError::ApiError(msg) => write!(f, "API error: {}", msg),
            GenerateEmailError::ExistingAccount(email) => {
                write!(f, "An existing account already exists. \nEmail: {}", email)
            }
        }
    }
}

impl Error for GenerateEmailError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GenerateEmailError::Request(e) => Some(e),
            GenerateEmailError::Io(e) => Some(e),
            GenerateEmailError::Serde(e) => Some(e),
            GenerateEmailError::ApiError(_) => None,
            GenerateEmailError::ExistingAccount(_) => None,
        }
    }
}

// Implement From traits for the error types
impl From<reqwest::Error> for GenerateEmailError {
    fn from(error: reqwest::Error) -> Self {
        GenerateEmailError::Request(error)
    }
}

impl From<std::io::Error> for GenerateEmailError {
    fn from(error: std::io::Error) -> Self {
        GenerateEmailError::Io(error)
    }
}

impl From<serde_json::Error> for GenerateEmailError {
    fn from(error: serde_json::Error) -> Self {
        GenerateEmailError::Serde(error)
    }
}
