use paying_attention_core::{DeclaredTask, Event};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Home,
    Office,
    Cafe,
    Library,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Energy {
    Exhausted,
    Low,
    Neutral,
    Focused,
    Flow,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CheckInInput {
    pub environment: Option<Environment>,
    pub energy: Option<Energy>,
    pub task_text: String,
}

impl CheckInInput {
    pub fn can_release(&self) -> bool {
        self.environment.is_some()
            && self.energy.is_some()
            && DeclaredTask::new(self.task_text.clone()).is_ok()
    }

    pub fn release_event(&self) -> Option<Event> {
        DeclaredTask::new(self.task_text.clone())
            .ok()
            .filter(|_| self.environment.is_some() && self.energy.is_some())
            .map(|declared_task| Event::CheckInSubmitted { declared_task })
    }
}
