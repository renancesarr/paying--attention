mod as_str;
mod new;

/// A non-empty task the user consciously commits to during an Attention Block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredTask(pub(crate) String);
