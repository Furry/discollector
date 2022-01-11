use std::fmt;

#[derive(Debug, Clone)]
pub struct DiscollectorError {
    message: Option<String>
}

impl fmt::Display for DiscollectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.message.is_some() {
            true => {
                let message = self.message
                    .clone()
                    .unwrap();
                write!(f, "{}", message.as_str())
            },
            _ => write!(f, "Unknown Discollector Error.")
        }
    }
}

impl Default for DiscollectorError {
    fn default() -> Self {
        Self { message: None }
    }
}