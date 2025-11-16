use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let kb = &app.config.keybinds;
    
    let help_text = vec![
        Line::from(vec![
            Span::styled("Remycord", Style::default().fg(theme.get_color("base0E")).add_modifier(Modifier::BOLD)),
            Span::raw(" - Discord TUI Client"),
        ]),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from(format!("  {} / {}  - Move up/down", kb.up.key, kb.down.key)),
        Line::from(format!("  {}      - Expand server / Select channel", kb.select.key)),
        Line::from(format!("  {}      - Go back / Quit", kb.quit.key)),
        Line::from(format!("  {}          - Settings", kb.settings.key)),
        Line::from(""),
        Line::from("In Messages:"),
        Line::from(format!("  {}          - Enter input mode", kb.input_mode.key)),
        Line::from(format!("  {}          - Attach file", kb.attach_file.key)),
        Line::from(format!("  {}      - Back to sidebar", kb.back.key)),
        Line::from(""),
        Line::from("Input Mode:"),
        Line::from(format!("  {}      - Send message", kb.send_message.key)),
        Line::from(format!("  {}      - Cancel", kb.cancel_input.key)),
        Line::from(format!("  {} / {}  - Move cursor", kb.cursor_left.key, kb.cursor_right.key)),
        Line::from(""),
        Line::from("Select a server to get started!"),
        Line::from(""),
        Line::from(Span::styled("(Using mock data - no Discord connection)", 
            Style::default().fg(theme.get_color("base03")))),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(theme.get_color("base05")));

    f.render_widget(help, area);
}
