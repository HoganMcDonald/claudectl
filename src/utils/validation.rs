use std::path::Path;
use std::env;
use owo_colors::OwoColorize;

pub type ValidationResult<T> = Result<T, ValidationError>;

#[derive(Debug)]
pub struct ValidationError {
    pub message: String,
}

impl ValidationError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
