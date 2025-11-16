#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Sidebar,
    Messages,
    Input,
    Settings,
    KeybindRecording(String),
}
