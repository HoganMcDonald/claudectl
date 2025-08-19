#[derive(Debug)]
pub struct CommandError {
    pub message: String,
}

impl CommandError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
