use crate::{DeclaredTask, EmptyDeclaredTask};

impl DeclaredTask {
    /// Create a task after validating the user's text.
    pub fn new(text: impl Into<String>) -> Result<Self, EmptyDeclaredTask> {
        let text = text.into();

        if text.trim().is_empty() {
            return Err(EmptyDeclaredTask);
        }

        Ok(Self(text))
    }
}
