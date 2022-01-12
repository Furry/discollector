use std::fmt;

#[derive(Debug, Clone)]
pub enum DiscollectorError {
    SerializationError(String),
    UnknownError
}

impl std::error::Error for DiscollectorError {}

impl fmt::Display for DiscollectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            DiscollectorError::UnknownError => "Unknown Error",
            _ => "N/A"
        };
        write!(f, "{}", c)
    }
}