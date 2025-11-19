mod sidebar;
mod messages;
mod settings;
mod help;
mod utils;
pub mod image;

use crate::app::{App, AppMode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};

pub fn draw(f: &mut Frame, app: &mut App) {
    match &app.mode {
        AppMode::Settings | AppMode::KeybindRecording(_) => {
            settings::draw(f, app);
        }
        _ => {
            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
                .split(f.area());

            sidebar::draw(f, app, main_chunks[0]);

            match app.mode {
                AppMode::Sidebar => {
                    help::draw(f, app, main_chunks[1]);
                }
                AppMode::Messages | AppMode::Input => {
                    messages::draw(f, app, main_chunks[1]);
                }
                _ => {}
            }
        }
    }
}
