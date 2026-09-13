//! Pure domain core for Paying Attention.
//!
//! This crate stays free of desktop, storage, clock, network, audio, and
//! filesystem dependencies. External adapters translate the outside world into
//! domain events and observe the Attention Workflow through its public seam.

mod attention_workflow;
mod declared_task;
mod empty_declared_task;
mod event;
mod invalid_transition;
mod workflow_view;

pub use attention_workflow::AttentionWorkflow;
pub use declared_task::DeclaredTask;
pub use empty_declared_task::EmptyDeclaredTask;
pub use event::Event;
pub use invalid_transition::InvalidTransition;
pub use workflow_view::{NaggingOrigin, WorkflowView};
