use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub created_at: Instant,
    pub duration: Duration,
    pub kind: NotificationKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationKind {
    Info,
    Success,
    Warning,
    Error,
}

impl Notification {
    pub fn new(message: impl Into<String>, kind: NotificationKind) -> Self {
        Self {
            message: message.into(),
            created_at: Instant::now(),
            duration: Duration::from_secs(3),
            kind,
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message, NotificationKind::Info)
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message, NotificationKind::Success)
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message, NotificationKind::Warning)
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message, NotificationKind::Error)
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }

    pub fn remaining_progress(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_secs_f32();
        let total = self.duration.as_secs_f32();
        (1.0 - (elapsed / total)).max(0.0).min(1.0)
    }
}
