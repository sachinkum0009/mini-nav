use std::io::Error;

/// # Mini Nav
pub struct MiniNav {}

impl MiniNav {
    /// Initialize new instance for Mini Nav
    pub fn new() -> Self {
        Self {}
    }
    pub fn publish() -> Result<bool, Error> {
        Ok(true)
    }
}
