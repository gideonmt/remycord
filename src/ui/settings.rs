use crate::app::{App, AppMode};
use ratatui::{
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use super::utils::centered_rect;

pub fn draw(f: &mut Frame, app: &App) {
    let theme = app.theme();
    let area = centered_rect(80, 80, f.area());
    
    let protocol_status = if app.image_renderer.is_supported() {
        format!("✓ {}", app.image_renderer.protocol_name())
    } else {
        format!("✗ {} (fallback only)", app.image_renderer.protocol_name())
    };
    
    let settings_items = vec![
        format!("Username: {}", app.config.general.username),
        format!("File Manager: {}", app.config.general.file_manager),
        format!("Show Timestamps: {}", if app.config.general.show_timestamps { "Yes" } else { "No" }),
        format!("Show Typing Indicators: {}", if app.config.general.show_typing_indicators { "Yes" } else { "No" }),
        format!("Message Scroll Speed: {}", app.config.general.message_scroll_speed),
        format!("Max Input Lines: {}", app.config.general.max_input_lines),
        format!("Theme: {}", app.config.theme_name),
        "".to_string(),
        format!("Image Protocol: {}", protocol_status),
        format!("Images Enabled: {}", if app.config.images.enabled { "Yes" } else { "No" }),
        format!("Render Avatars: {}", if app.config.images.render_avatars { "Yes" } else { "No" }),
        format!("Render Custom Emojis: {}", if app.config.images.render_emojis { "Yes" } else { "No" }),
        format!("Render Stickers: {}", if app.config.images.render_stickers { "Yes" } else { "No" }),
        format!("Render Attachments: {}", if app.config.images.render_attachments { "Yes" } else { "No" }),
        format!("Render Server Icons: {}", if app.config.images.render_server_icons { "Yes" } else { "No" }),
        format!("Max Image Width: {} cols", app.config.images.max_image_width),
        format!("Max Image Height: {} rows", app.config.images.max_image_height),
        "".to_string(),
        "Keybinds:".to_string(),
        format!("  Quit: {}{}", format_modifiers(&app.config.keybinds.quit.modifiers), app.config.keybinds.quit.key),
        format!("  Settings: {}{}", format_modifiers(&app.config.keybinds.settings.modifiers), app.config.keybinds.settings.key),
        format!("  Up: {}{}", format_modifiers(&app.config.keybinds.up.modifiers), app.config.keybinds.up.key),
        format!("  Down: {}{}", format_modifiers(&app.config.keybinds.down.modifiers), app.config.keybinds.down.key),
        format!("  Select: {}{}", format_modifiers(&app.config.keybinds.select.modifiers), app.config.keybinds.select.key),
        format!("  Back: {}{}", format_modifiers(&app.config.keybinds.back.modifiers), app.config.keybinds.back.key),
        format!("  Input Mode: {}{}", format_modifiers(&app.config.keybinds.input_mode.modifiers), app.config.keybinds.input_mode.key),
        format!("  Attach File: {}{}", format_modifiers(&app.config.keybinds.attach_file.modifiers), app.config.keybinds.attach_file.key),
        format!("  Send Message: {}{}", format_modifiers(&app.config.keybinds.send_message.modifiers), app.config.keybinds.send_message.key),
    ];

    let list_items: Vec<ListItem> = settings_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.settings_selected {
                Style::default()
                    .fg(theme.get_color("base0A"))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.get_color("base05"))
            };
            ListItem::new(item.as_str()).style(style)
        })
        .collect();

    let title = if let AppMode::KeybindRecording(action) = &app.mode {
        format!("Settings - Recording keybind for: {} (Press any key)", action)
    } else {
        "Settings (↑/↓: navigate, Enter: edit, r: record keybind, Esc: close)".to_string()
    };

    let list = List::new(list_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(theme.get_color("base0D")))
    );

    f.render_widget(list, area);
}

fn format_modifiers(modifiers: &[String]) -> String {
    if modifiers.is_empty() {
        String::new()
    } else {
        format!("{}+", modifiers.join("+"))
    }
}
