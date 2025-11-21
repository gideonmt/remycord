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

    let author_color = theme.get_color("base0E");
    let time_color = theme.get_color("base03");
    let text_color = theme.get_color("base05");
    let attachment_color = theme.get_color("base0C");
    let dim_color = theme.get_color("base04");
    let border_color = theme.get_color("base03");

    let mut typing_line = Vec::new();
    if app.config.general.show_typing_indicators && !app.typing_users.is_empty() {
        let typing_text = if app.typing_users.len() == 1 {
            format!("{} is typing...", app.typing_users[0])
        } else {
            format!("{} are typing...", app.typing_users.join(", "))
        };
        typing_line.push(Line::from(vec![
            Span::styled(typing_text, Style::default().fg(time_color).add_modifier(Modifier::ITALIC)),
        ]));
        typing_line.push(Line::from(""));
    }

    let avatar_width = if show_avatars { 5 } else { 0 };
    let avatar_height = 4;
    let messages_inner = Block::default()
        .borders(Borders::ALL)
        .title("Messages")
        .border_style(Style::default().fg(border_color))
        .inner(area);

    let available_image_width = messages_inner.width.saturating_sub(avatar_width + 4);

    let mut current_y = messages_inner.y;
    let messages_to_show = app
        .messages
        .iter()
        .skip(app.message_scroll)
        .collect::<Vec<_>>();

    for msg in messages_to_show {
        if current_y >= messages_inner.bottom() {
            break;
        }

        let content_lines = if msg.content.is_empty() { 0 } else { msg.content.lines().count() };
        let has_images = show_attachments && msg.has_images();
        let image_attachments: Vec<_> = if has_images {
            msg.attachments.iter().filter(|a| a.is_image()).collect()
        } else {
            Vec::new()
        };
        
        let header_height = 1;
        let content_height = if content_lines > 0 { content_lines + 1 } else { 0 };
        let attachment_text_height = if !msg.attachments.is_empty() { 1 + msg.attachments.len() } else { 0 };
        
        let image_height = if show_attachments && !image_attachments.is_empty() {
            image_attachments.len() * (app.config.images.max_image_height as usize + 1)
        } else {
            0
        };
        
        let text_total_height = header_height + content_height + attachment_text_height;
        let total_height = text_total_height + image_height;
        let remaining_height = messages_inner.bottom().saturating_sub(current_y);
        let msg_height = total_height.min(remaining_height as usize) as u16;

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

        if show_avatars {
            if let Some(protocol) = app.image_renderer.get_avatar(&msg.author_id) {
                let avatar_area = Rect {
                    x: msg_chunks[0].x,
                    y: msg_chunks[0].y,
                    width: msg_chunks[0].width,
                    height: avatar_height.min(msg_chunks[0].height),
                };
                let image_widget = StatefulImage::default();
                f.render_stateful_widget(image_widget, avatar_area, protocol);
            }
        }

        let message_area = if show_avatars { msg_chunks[1] } else { msg_chunks[0] };
        
        let author_style = Style::default().fg(author_color).add_modifier(Modifier::BOLD);
        let time_style = Style::default().fg(time_color);
        
        let mut header_spans = vec![];
        if app.config.general.show_timestamps {
            header_spans.push(Span::styled(format!("[{}] ", msg.timestamp), time_style));
        }
        header_spans.push(Span::styled(&msg.author, author_style));
        
        let mut message_lines = vec![Line::from(header_spans)];
        
        if !msg.content.is_empty() {
            for line in msg.content.lines() {
                message_lines.push(Line::from(Span::styled(line.to_string(), Style::default().fg(text_color))));
            }
            message_lines.push(Line::from(""));
        }
        
        if !msg.attachments.is_empty() {
            for attachment in msg.attachments.iter() {
                if attachment.is_image() {
                    if show_attachments {
                        message_lines.push(Line::from(vec![
                            Span::styled("i  ", Style::default().fg(attachment_color)),
                            Span::styled(&attachment.filename, Style::default().fg(attachment_color)),
                        ]));
                    } else {
                        message_lines.push(Line::from(Span::styled(
                            format!("i  {} (images disabled)", attachment.filename),
                            Style::default().fg(dim_color),
                        )));
                    }
                } else {
                    message_lines.push(Line::from(Span::styled(
                        format!("f {}", attachment.filename),
                        Style::default().fg(attachment_color),
                    )));
                }
            }
            message_lines.push(Line::from(""));
        }
        
        let text_height = message_lines.len() as u16;
        let text_area = Rect {
            x: message_area.x,
            y: message_area.y,
            width: message_area.width,
            height: text_height.min(message_area.height),
        };
        
        let message_para = Paragraph::new(message_lines).wrap(Wrap { trim: false });
        f.render_widget(message_para, text_area);
        
        if show_attachments && !image_attachments.is_empty() {
            let mut image_y = text_area.y + text_height;
            let max_img_height = app.config.images.max_image_height;
            
            for attachment in image_attachments.iter() {
                if image_y >= message_area.bottom() {
                    break;
                }
                
                if let Some(protocol) = app.image_renderer.get_attachment(&attachment.id) {
                    let available_height = message_area.bottom().saturating_sub(image_y);
                    let img_height = max_img_height.min(available_height);
                
                    let image_area = Rect {
                        x: message_area.x,
                        y: image_y,
                        width: available_image_width.min(message_area.width),
                        height: img_height,
                    };
                    let image_widget = StatefulImage::default();
                    f.render_stateful_widget(image_widget, image_area, protocol);
                    image_y += img_height + 1;
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
        .block(Block::default().borders(Borders::ALL).title(input_title).border_style(input_style))
        .style(Style::default().fg(theme.get_color("base05")))
        .wrap(Wrap { trim: false });
    f.render_widget(input, area);

    if app.mode == AppMode::Input {
        let cursor_x = area.x + 1 + (app.input_cursor as u16 % (area.width - 2));
        let cursor_y = area.y + 1 + (app.input_cursor as u16 / (area.width - 2));
        f.set_cursor_position((cursor_x, cursor_y));
    }
}
