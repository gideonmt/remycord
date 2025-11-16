use crate::app::App;
use crate::models::NotificationKind;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    if app.notifications.is_empty() {
        return;
    }

    let theme = app.theme();
    
    let notification_width = 40.min(area.width / 3);
    let notification_height = 3;
    
    for (i, notification) in app.notifications.iter().enumerate().take(5) {
        let y_offset = i as u16 * (notification_height + 1);
        
        if y_offset + notification_height > area.height {
            break;
        }
        
        let notification_area = Rect {
            x: area.width.saturating_sub(notification_width + 1),
            y: area.y + y_offset + 1,
            width: notification_width,
            height: notification_height,
        };
        
        let (color, icon) = match notification.kind {
            NotificationKind::Info => (theme.get_color("base0D"), "ℹ"),
            NotificationKind::Success => (theme.get_color("base0B"), "✓"),
            NotificationKind::Warning => (theme.get_color("base0A"), "⚠"),
            NotificationKind::Error => (theme.get_color("base08"), "✗"),
        };
        
        let progress = notification.remaining_progress();
        let bar_width = (notification_width.saturating_sub(4) as f32 * progress) as usize;
        let progress_bar = "─".repeat(bar_width);
        
        let text = vec![
            Line::from(vec![
                Span::styled(format!("{} ", icon), Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::styled(&notification.message, Style::default().fg(theme.get_color("base05"))),
            ]),
            Line::from(Span::styled(progress_bar, Style::default().fg(color))),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color))
            )
            .alignment(Alignment::Left);
        
        f.render_widget(paragraph, notification_area);
    }
}
