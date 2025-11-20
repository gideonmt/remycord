use anyhow::Result;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use crate::app::{App, AppMode, SidebarItem};
use crate::config::{Keybinds, KeyBind, save_config, get_available_themes};
use crate::models::{Notification, ChannelType};
use super::file_picker;

pub fn handle_keybind_recording(app: &mut App, key: KeyEvent, action: &str) -> Result<bool> {
    let new_keybind = KeyBind {
        key: format!("{:?}", key.code)
            .replace("Char('", "")
            .replace("')", ""),
        modifiers: {
            let mut mods = Vec::new();
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                mods.push("Ctrl".to_string());
            }
            if key.modifiers.contains(KeyModifiers::ALT) {
                mods.push("Alt".to_string());
            }
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                mods.push("Shift".to_string());
            }
            mods
        },
    };

    match action {
        "Quit" => app.config.keybinds.quit = new_keybind,
        "Settings" => app.config.keybinds.settings = new_keybind,
        "Up" => app.config.keybinds.up = new_keybind,
        "Down" => app.config.keybinds.down = new_keybind,
        "Select" => app.config.keybinds.select = new_keybind,
        "Back" => app.config.keybinds.back = new_keybind,
        "Input Mode" => app.config.keybinds.input_mode = new_keybind,
        "Attach File" => app.config.keybinds.attach_file = new_keybind,
        "Send Message" => app.config.keybinds.send_message = new_keybind,
        _ => {}
    }

    save_config(&app.config)?;
    app.mode = AppMode::Settings;
    Ok(false)
}

pub fn handle_sidebar_input(app: &mut App, key: KeyEvent, kb: &Keybinds) {
    if kb.down.matches(key.code, key.modifiers) || key.code == KeyCode::Down {
        navigate_sidebar_down(app);
    } else if kb.up.matches(key.code, key.modifiers) || key.code == KeyCode::Up {
        navigate_sidebar_up(app);
    } else if kb.select.matches(key.code, key.modifiers) {
        select_sidebar_item(app);
    } else if kb.settings.matches(key.code, key.modifiers) {
        app.mode = AppMode::Settings;
    }
}

pub fn handle_messages_input(
    app: &mut App,
    key: KeyEvent,
    kb: &Keybinds,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<()> {
    if kb.scroll_down.matches(key.code, key.modifiers) || key.code == KeyCode::Down {
        scroll_messages_down(app);
    } else if kb.scroll_up.matches(key.code, key.modifiers) || key.code == KeyCode::Up {
        scroll_messages_up(app);
    } else if kb.input_mode.matches(key.code, key.modifiers) {
        app.mode = AppMode::Input;
    } else if kb.attach_file.matches(key.code, key.modifiers) {
        file_picker::pick_file(app, terminal)?;
    } else if kb.settings.matches(key.code, key.modifiers) {
        app.mode = AppMode::Settings;
    }
    Ok(())
}

pub fn handle_input_mode(app: &mut App, key: KeyEvent, kb: &Keybinds) {
    if kb.cancel_input.matches(key.code, key.modifiers) {
        app.mode = AppMode::Messages;
        app.attached_files.clear();
    } else if kb.send_message.matches(key.code, key.modifiers) {
        app.mode = AppMode::Messages;
    } else if let KeyCode::Char(c) = key.code {
        if !key.modifiers.contains(KeyModifiers::CONTROL) {
            app.input.insert(app.input_cursor, c);
            app.input_cursor += 1;
        }
    } else if key.code == KeyCode::Backspace {
        if app.input_cursor > 0 {
            app.input.remove(app.input_cursor - 1);
            app.input_cursor -= 1;
        }
    } else if kb.cursor_left.matches(key.code, key.modifiers) || key.code == KeyCode::Left {
        if app.input_cursor > 0 {
            app.input_cursor -= 1;
        }
    } else if kb.cursor_right.matches(key.code, key.modifiers) || key.code == KeyCode::Right {
        if app.input_cursor < app.input.len() {
            app.input_cursor += 1;
        }
    } else if kb.cursor_start.matches(key.code, key.modifiers) || key.code == KeyCode::Home {
        app.input_cursor = 0;
    } else if kb.cursor_end.matches(key.code, key.modifiers) || key.code == KeyCode::End {
        app.input_cursor = app.input.len();
    }
}

pub fn handle_settings_input(app: &mut App, key: KeyEvent, kb: &Keybinds) -> Result<()> {
    if key.code == KeyCode::Down || kb.down.matches(key.code, key.modifiers) {
        if app.settings_selected < 27 { // Updated max index
            app.settings_selected += 1;
        }
    } else if key.code == KeyCode::Up || kb.up.matches(key.code, key.modifiers) {
        if app.settings_selected > 0 {
            app.settings_selected -= 1;
        }
    } else if key.code == KeyCode::Enter {
        edit_setting(app)?;
    } else if key.code == KeyCode::Char('r') {
        start_keybind_recording(app);
    }
    Ok(())
}

fn navigate_sidebar_down(app: &mut App) {
    let items = app.get_sidebar_items();
    if !items.is_empty() {
        app.selected_sidebar_idx = (app.selected_sidebar_idx + 1) % items.len();
    }
}

fn navigate_sidebar_up(app: &mut App) {
    let items = app.get_sidebar_items();
    if !items.is_empty() {
        if app.selected_sidebar_idx > 0 {
            app.selected_sidebar_idx -= 1;
        } else {
            app.selected_sidebar_idx = items.len() - 1;
        }
    }
}

fn select_sidebar_item(app: &mut App) {
    let items = app.get_sidebar_items();
    if let Some(item) = items.get(app.selected_sidebar_idx) {
        match item {
            SidebarItem::DmSection => {
                app.dm_section_expanded = !app.dm_section_expanded;
                if app.dm_section_expanded && app.dms.is_empty() {
                    app.loading_dms = true;
                }
            }
            SidebarItem::DmChannel(dm) => {
                app.selected_channel = Some(dm.id.clone());
                app.message_scroll = 0;
                
                if let Some(messages) = app.message_cache.get(&dm.id) {
                    app.messages = messages.clone();
                } else {
                    app.messages.clear();
                    app.loading_messages = true;
                }
                
                app.mode = AppMode::Messages;
            }
            SidebarItem::ServerSection => {
                // Do nothing, just a label
            }
            SidebarItem::Server(guild) => {
                if let Some(g) = app.guilds.iter_mut().find(|g| g.id == guild.id) {
                    g.toggle_expanded();
                    
                    if g.expanded && !app.channel_cache.contains_key(&g.id) {
                        app.loading_channels = true;
                    }
                }
            }
            SidebarItem::Category { guild_id, category, .. } => {
                app.toggle_category(guild_id, &category.id);
            }
            SidebarItem::Channel { channel, .. } => {
                if !channel.is_text_based() {
                    app.add_notification(Notification::info(
                        format!("Cannot view messages in {} channels", 
                            match channel.kind {
                                ChannelType::Voice => "voice",
                                ChannelType::Stage => "stage",
                                _ => "this type of",
                            }
                        )
                    ));
                    return;
                }
                
                app.selected_channel = Some(channel.id.clone());
                app.message_scroll = 0;
                
                if let Some(messages) = app.message_cache.get(&channel.id) {
                    app.messages = messages.clone();
                } else {
                    app.messages.clear();
                    app.loading_messages = true;
                }
                
                app.mode = AppMode::Messages;
            }
        }
    }
}

pub fn exit_channel(app: &mut App) {
    app.mode = AppMode::Sidebar;
    app.selected_channel = None;
    app.messages.clear();
    app.message_scroll = 0;
    app.attached_files.clear();
}

fn scroll_messages_down(app: &mut App) {
    let speed = app.config.general.message_scroll_speed;
    if app.message_scroll < app.messages.len().saturating_sub(1) {
        app.message_scroll = (app.message_scroll + speed)
            .min(app.messages.len().saturating_sub(1));
    }
}

fn scroll_messages_up(app: &mut App) {
    let speed = app.config.general.message_scroll_speed;
    if app.message_scroll > 0 {
        app.message_scroll = app.message_scroll.saturating_sub(speed);
    }
}

fn edit_setting(app: &mut App) -> Result<()> {
    match app.settings_selected {
        1 => {
            app.config.general.file_manager = if app.config.general.file_manager == "fzf" {
                "lf".to_string()
            } else {
                "fzf".to_string()
            };
        }
        2 => app.config.general.show_timestamps = !app.config.general.show_timestamps,
        3 => app.config.general.show_typing_indicators = !app.config.general.show_typing_indicators,
        4 => app.config.general.message_scroll_speed = (app.config.general.message_scroll_speed % 5) + 1,
        5 => {
            app.config.general.max_input_lines = if app.config.general.max_input_lines >= 12 {
                4
            } else {
                app.config.general.max_input_lines + 1
            };
        }
        6 => cycle_theme(app)?,
        // Skip 7 (empty line)
        // Skip 8 (Image Support - read-only)
        9 => app.config.images.enabled = !app.config.images.enabled,
        10 => app.config.images.render_avatars = !app.config.images.render_avatars,
        11 => app.config.images.render_emojis = !app.config.images.render_emojis,
        12 => app.config.images.render_stickers = !app.config.images.render_stickers,
        13 => app.config.images.render_attachments = !app.config.images.render_attachments,
        14 => app.config.images.render_server_icons = !app.config.images.render_server_icons,
        15 => {
            app.config.images.max_image_width = match app.config.images.max_image_width {
                10 => 20,
                20 => 30,
                30 => 40,
                40 => 50,
                _ => 10,
            };
        }
        16 => {
            app.config.images.max_image_height = match app.config.images.max_image_height {
                5 => 10,
                10 => 15,
                15 => 20,
                20 => 25,
                _ => 5,
            };
        }
        _ => {}
    }
    
    save_config(&app.config)?;
    Ok(())
}

fn cycle_theme(app: &mut App) -> Result<()> {
    if let Ok(themes) = get_available_themes() {
        if !themes.is_empty() {
            if let Some(current_idx) = themes.iter().position(|name| name == &app.config.theme_name) {
                let next_idx = (current_idx + 1) % themes.len();
                app.config.theme_name = themes[next_idx].clone();
                app.reload_theme();
            }
        }
    }
    Ok(())
}

fn start_keybind_recording(app: &mut App) {
    let action = match app.settings_selected {
        19 => Some("Quit"),
        20 => Some("Settings"),
        21 => Some("Up"),
        22 => Some("Down"),
        23 => Some("Select"),
        24 => Some("Back"),
        25 => Some("Input Mode"),
        26 => Some("Attach File"),
        27 => Some("Send Message"),
        _ => None,
    };
    
    if let Some(action) = action {
        app.mode = AppMode::KeybindRecording(action.to_string());
    }
}
