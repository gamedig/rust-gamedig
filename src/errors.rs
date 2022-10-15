use core::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum GDError {
    IDK(String)
}

impl fmt::Display for GDError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GDError::IDK(details) => write!(f, "IDK: {details}")
        }
    }
}
