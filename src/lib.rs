use std::{fmt, io};

pub mod load;
pub mod tui;

#[derive(Debug, Clone)]
pub struct ChecklistError {
    msg: String,
}

impl ChecklistError {
    pub fn from<T>(msg: T) -> Self
    where
        T: ToString,
    {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for ChecklistError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.msg)
    }
}

impl std::error::Error for ChecklistError {}

impl From<io::Error> for ChecklistError {
    fn from(value: io::Error) -> Self {
        ChecklistError {
            msg: format!("{:?}", value),
        }
    }
}
