use crate::DeclaredTask;

impl DeclaredTask {
    /// Return the task text as declared by the user.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
