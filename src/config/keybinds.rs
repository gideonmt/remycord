use crossterm::event::{KeyCode, KeyModifiers};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybinds {
    pub quit: KeyBind,
    pub settings: KeyBind,
    pub up: KeyBind,
    pub down: KeyBind,
    pub select: KeyBind,
    pub back: KeyBind,
    pub input_mode: KeyBind,
    pub attach_file: KeyBind,
    pub scroll_up: KeyBind,
    pub scroll_down: KeyBind,
    pub send_message: KeyBind,
    pub cancel_input: KeyBind,
    pub cursor_left: KeyBind,
    pub cursor_right: KeyBind,
    pub cursor_start: KeyBind,
    pub cursor_end: KeyBind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBind {
    pub key: String,
    pub modifiers: Vec<String>,
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            quit: KeyBind::new("q", vec![]),
            settings: KeyBind::new("s", vec![]),
            up: KeyBind::new("k", vec![]),
            down: KeyBind::new("j", vec![]),
            select: KeyBind::new("Enter", vec![]),
            back: KeyBind::new("Esc", vec![]),
            input_mode: KeyBind::new("i", vec![]),
            attach_file: KeyBind::new("a", vec![]),
            scroll_up: KeyBind::new("k", vec![]),
            scroll_down: KeyBind::new("j", vec![]),
            send_message: KeyBind::new("Enter", vec![]),
            cancel_input: KeyBind::new("Esc", vec![]),
            cursor_left: KeyBind::new("Left", vec![]),
            cursor_right: KeyBind::new("Right", vec![]),
            cursor_start: KeyBind::new("a", vec!["Ctrl"]),
            cursor_end: KeyBind::new("e", vec!["Ctrl"]),
        }
    }
}

impl KeyBind {
    pub fn new(key: &str, modifiers: Vec<&str>) -> Self {
        Self {
            key: key.to_string(),
            modifiers: modifiers.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn matches(&self, code: KeyCode, mods: KeyModifiers) -> bool {
        let key_matches = match code {
            KeyCode::Char(c) => {
                let c_str = c.to_string();
                self.key == c_str || self.key.to_lowercase() == c_str.to_lowercase()
            },
            KeyCode::Enter => self.key == "Enter",
            KeyCode::Esc => self.key == "Esc",
            KeyCode::Backspace => self.key == "Backspace",
            KeyCode::Left => self.key == "Left",
            KeyCode::Right => self.key == "Right",
            KeyCode::Up => self.key == "Up",
            KeyCode::Down => self.key == "Down",
            KeyCode::Home => self.key == "Home",
            KeyCode::End => self.key == "End",
            KeyCode::Tab => self.key == "Tab",
            _ => false,
        };

        if !key_matches {
            return false;
        }

        let has_ctrl = mods.contains(KeyModifiers::CONTROL);
        let has_alt = mods.contains(KeyModifiers::ALT);
        let has_shift = mods.contains(KeyModifiers::SHIFT);

        let needs_ctrl = self.modifiers.contains(&"Ctrl".to_string());
        let needs_alt = self.modifiers.contains(&"Alt".to_string());
        let needs_shift = self.modifiers.contains(&"Shift".to_string());

        has_ctrl == needs_ctrl && has_alt == needs_alt && has_shift == needs_shift;

        let matches = has_ctrl == needs_ctrl && has_alt == needs_alt && has_shift == needs_shift;
        
        // DEBUG OUTPUT
        eprintln!("KeyBind check: key='{}' expected='{}' key_matches={} has_ctrl={} needs_ctrl={} result={}",
            format!("{:?}", code), self.key, key_matches, has_ctrl, needs_ctrl, matches);

        matches
    }
}
