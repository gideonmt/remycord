mod handlers;
mod file_picker;

use anyhow::Result;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use crate::app::{App, AppMode};
use crate::config::save_config;

pub fn handle_input(
    app: &mut App,
    key: KeyEvent,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<bool> {
    // DEBUG: Print what key was pressed
    eprintln!("Key pressed: {:?}, Modifiers: {:?}, Current mode: {:?}", key.code, key.modifiers, app.mode);
    
    if let AppMode::KeybindRecording(action) = &app.mode.clone() {
        return handlers::handle_keybind_recording(app, key, action);
    }

    let kb = &app.config.keybinds.clone();
    
    // Handle Ctrl+C globally - always quit
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        eprintln!("Ctrl+C detected - quitting!");
        return Ok(true);
    }
    
    let should_quit = kb.quit.matches(key.code, key.modifiers);
    eprintln!("Should quit: {}", should_quit);

    match &app.mode {
        AppMode::Sidebar => {
            if should_quit {
                eprintln!("Quitting from sidebar!");
                return Ok(true);
            }
            handlers::handle_sidebar_input(app, key, kb);
        }
        AppMode::Messages => {
            if should_quit || kb.back.matches(key.code, key.modifiers) {
                eprintln!("Exiting channel");
                handlers::exit_channel(app);
            } else {
                handlers::handle_messages_input(app, key, kb, terminal)?;
            }
        }
        AppMode::Input => {
            handlers::handle_input_mode(app, key, kb);
        }
        AppMode::Settings => {
            if should_quit || key.code == KeyCode::Esc {
                eprintln!("Exiting settings");
                save_config(&app.config)?;
                app.mode = if app.selected_channel.is_some() {
                    AppMode::Messages
                } else {
                    AppMode::Sidebar
                };
            } else {
                handlers::handle_settings_input(app, key, kb)?;
            }
        }
        AppMode::KeybindRecording(_) => {}
    }

    Ok(false)
}

