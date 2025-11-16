use crate::app::{App, AppMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let input_lines = app.input.lines().count().max(1).min(app.config.general.max_input_lines);
    let input_height = (input_lines + 2) as u16;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(input_height),
        ])
        .split(area);

    draw_header(f, app, chunks[0]);
    draw_messages_area(f, app, chunks[1]);
    draw_input(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let channel_name = app.get_current_channel_name().unwrap_or_else(|| "Unknown".to_string());
    let guild_name = app.get_current_guild_name().unwrap_or_else(|| "Unknown".to_string());

    let header = Paragraph::new(format!("{} > #{}", guild_name, channel_name))
        .block(Block::default().borders(Borders::ALL).title("Channel").border_style(Style::default().fg(theme.get_color("base03"))))
        .style(Style::default().fg(theme.get_color("base0B")));
    f.render_widget(header, area);
}

fn draw_messages_area(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    
    let mut typing_line = Vec::new();
    if app.config.general.show_typing_indicators && !app.typing_users.is_empty() {
        let typing_text = if app.typing_users.len() == 1 {
            format!("{} is typing...", app.typing_users[0])
        } else {
            format!("{} are typing...", app.typing_users.join(", "))
        };
        typing_line.push(Line::from(vec![
            Span::styled(typing_text, Style::default().fg(theme.get_color("base03")).add_modifier(Modifier::ITALIC)),
        ]));
        typing_line.push(Line::from(""));
    }

    let mut messages_text: Vec<Line> = app
        .messages
        .iter()
        .skip(app.message_scroll)
        .flat_map(|msg| {
            let author_style = Style::default()
                .fg(theme.get_color("base0E"))
                .add_modifier(Modifier::BOLD);
            let time_style = Style::default().fg(theme.get_color("base03"));
            
            let mut header_spans = vec![];
            if app.config.general.show_timestamps {
                header_spans.push(Span::styled(format!("[{}] ", msg.timestamp), time_style));
            }
            header_spans.push(Span::styled(&msg.author, author_style));
            
            let header = Line::from(header_spans);

            let mut lines = vec![header];
            
            for line in msg.content.lines() {
                lines.push(Line::from(Span::styled(format!("  {}", line), Style::default().fg(theme.get_color("base05")))));
            }
            
            lines.push(Line::from(""));
            lines
        })
        .collect();

    messages_text.extend(typing_line);

    let messages = Paragraph::new(messages_text)
        .block(Block::default().borders(Borders::ALL).title("Messages").border_style(Style::default().fg(theme.get_color("base03"))))
        .wrap(Wrap { trim: false });
    f.render_widget(messages, area);
}

fn draw_input(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();

    let input_style = if app.mode == AppMode::Input {
        Style::default().fg(theme.get_color("base0A"))
    } else {
        Style::default().fg(theme.get_color("base03"))
    };

    let mut input_title = if app.mode == AppMode::Input {
        format!("Input ({} to send, {} to cancel)", app.config.keybinds.send_message.key, app.config.keybinds.cancel_input.key)
    } else {
        format!("Input (press '{}' to type, '{}' to attach file)", app.config.keybinds.input_mode.key, app.config.keybinds.attach_file.key)
    };

    if !app.attached_files.is_empty() {
        input_title.push_str(&format!(" [{} file(s)]", app.attached_files.len()));
    }

    let input = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(input_title)
                .border_style(input_style),
        )
        .style(Style::default().fg(theme.get_color("base05")))
        .wrap(Wrap { trim: false });
    f.render_widget(input, area);

    if app.mode == AppMode::Input {
        let cursor_x = area.x + 1 + (app.input_cursor as u16 % (area.width - 2));
        let cursor_y = area.y + 1 + (app.input_cursor as u16 / (area.width - 2));
        f.set_cursor_position((cursor_x, cursor_y));
    }
}
