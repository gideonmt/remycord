#[derive(Debug, Clone)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub expanded: bool,
}

impl Guild {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            expanded: false,
        }
    }

    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}
