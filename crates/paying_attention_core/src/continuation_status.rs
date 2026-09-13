/// Maximum number of times a Declared Task may be continued after its original declaration.
pub const CONTINUATION_LIMIT: u8 = 2;

/// Read-only Continuation information shown during Review.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContinuationStatus {
    pub used: u8,
    pub limit: u8,
}
