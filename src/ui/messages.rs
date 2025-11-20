use crate::app::{App, AppMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use ratatui_image::StatefulImage;

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
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

fn draw_messages_area(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = app.theme();
    let show_avatars = app.config.images.enabled && app.config.images.render_avatars;
    let show_attachments = app.config.images.enabled && app.config.images.render_attachments;
    
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

    let avatar_width = if show_avatars { 5 } else { 0 };
    let messages_inner = Block::default()
        .borders(Borders::ALL)
        .title("Messages")
        .border_style(Style::default().fg(theme.get_color("base03")))
        .inner(area);

    // Collect theme colors we'll need
    let author_color = theme.get_color("base0E");
    let time_color = theme.get_color("base03");
    let text_color = theme.get_color("base05");
    let border_color = theme.get_color("base03");
    let attachment_color = theme.get_color("base0C");

    // Draw messages
    let mut current_y = messages_inner.y;
    let messages_to_show = app
        .messages
        .iter()
        .skip(app.message_scroll)
        .take_while(|_| current_y < messages_inner.bottom())
        .collect::<Vec<_>>();

    for msg in messages_to_show {
        if current_y >= messages_inner.bottom() {
            break;
        }

        // Calculate height needed for this message
        let content_lines = msg.content.lines().count().max(1);
        let has_images = show_attachments && msg.has_images();
        let image_count = if has_images {
            msg.attachments.iter().filter(|a| a.is_image()).count()
        } else {
            0
        };
        
        let base_height = 2 + content_lines;
        let image_height = if image_count > 0 {
            (app.config.images.max_image_height + 1) * image_count as u16
        } else {
            0
        };
        let msg_height = (base_height as u16 + image_height).min(messages_inner.bottom() - current_y);

        let msg_chunks = if show_avatars {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(avatar_width),
                    Constraint::Min(0),
                ])
                .split(Rect {
                    x: messages_inner.x,
                    y: current_y,
                    width: messages_inner.width,
                    height: msg_height,
                })
                .to_vec()
        } else {
            vec![Rect {
                x: messages_inner.x,
                y: current_y,
                width: messages_inner.width,
                height: msg_height,
            }]
        };

        // Draw avatar
        if show_avatars {
            if let Some(avatar_protocol) = app.image_renderer.get_avatar(&msg.author_id) {
                let image = StatefulImage::default();
                let avatar_area = Rect {
                    x: msg_chunks[0].x,
                    y: msg_chunks[0].y,
                    width: msg_chunks[0].width,
                    height: 4.min(msg_chunks[0].height),
                };
                f.render_stateful_widget(image, avatar_area, avatar_protocol);
            }
        }

        // Draw message content
        let message_area = if show_avatars { msg_chunks[1] } else { msg_chunks[0] };
        
        let author_style = Style::default()
            .fg(author_color)
            .add_modifier(Modifier::BOLD);
        let time_style = Style::default().fg(time_color);
        
        let mut header_spans = vec![];
        if app.config.general.show_timestamps {
            header_spans.push(Span::styled(format!("[{}] ", msg.timestamp), time_style));
        }
        header_spans.push(Span::styled(&msg.author, author_style));
        
        let mut message_lines = vec![Line::from(header_spans)];
        
        for line in msg.content.lines() {
            message_lines.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(text_color)
            )));
        }
        
        // Add attachment indicators
        if !msg.attachments.is_empty() {
            message_lines.push(Line::from(""));
            for attachment in &msg.attachments {
                if attachment.is_image() {
                    if show_attachments {
                        message_lines.push(Line::from(Span::styled(
                            format!("📎 {}", attachment.filename),
                            Style::default().fg(attachment_color)
                        )));
                    } else {
                        message_lines.push(Line::from(Span::styled(
                            format!("🖼️  {} (images disabled)", attachment.filename),
                            Style::default().fg(attachment_color)
                        )));
                    }
                } else {
                    message_lines.push(Line::from(Span::styled(
                        format!("📎 {}", attachment.filename),
                        Style::default().fg(attachment_color)
                    )));
                }
            }
        }
        
        message_lines.push(Line::from(""));
        
        let text_height = message_lines.len() as u16;
        let text_area = Rect {
            x: message_area.x,
            y: message_area.y,
            width: message_area.width,
            height: text_height.min(message_area.height),
        };
        
        let message_para = Paragraph::new(message_lines)
            .wrap(Wrap { trim: false });
        
        f.render_widget(message_para, text_area);
        
        // Draw image attachments
        if show_attachments && has_images {
            let mut image_y = text_area.y + text_height;
            
            for attachment in msg.attachments.iter().filter(|a| a.is_image()) {
                if image_y >= message_area.bottom() {
                    break;
                }
                
                if let Some(image_protocol) = app.image_renderer.get_attachment(&attachment.id) {
                    let image_area = Rect {
                        x: message_area.x,
                        y: image_y,
                        width: app.config.images.max_image_width.min(message_area.width),
                        height: app.config.images.max_image_height.min(message_area.bottom() - image_y),
                    };
                    
                    let image = StatefulImage::default();
                    f.render_stateful_widget(image, image_area, image_protocol);
                    
                    image_y += image_area.height + 1;
                }
            }
        }
        
        current_y += msg_height;
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Messages")
        .border_style(Style::default().fg(border_color));
    f.render_widget(block, area);
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
