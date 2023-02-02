use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
    header: Option<String>,
}

impl Error {
    pub fn new(message: String, header: Option<String>) -> Error {
        Error { message, header }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}")
        match &self.header {
            None => write!(f, "PSO Handler Error: {}", self.message),
            Some(v) => write!(f, "PSO Handler Error: {} == Header: {}", self.message, v),
        }
    }
}
