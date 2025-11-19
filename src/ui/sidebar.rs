use crate::app::{App, AppMode, SidebarItem};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App, area: Rect) {
    let items = app.get_sidebar_items();
    let theme = app.theme();
    
    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let text = match item {
                SidebarItem::DmSection => {
                    let arrow = if app.dm_section_expanded { "▼ " } else { "▶ " };
                    format!("{}Direct Messages", arrow)
                }
                SidebarItem::DmChannel(dm) => {
                    format!("  # {}", dm.display_name())
                }
                SidebarItem::ServerSection => {
                    "▼ Servers".to_string()
                }
                SidebarItem::Server(guild) => {
                    let arrow = if guild.expanded { "▼ " } else { "▶ " };
                    format!("  {}{}", arrow, guild.name)
                }
                SidebarItem::Channel(channel) => {
                    format!("    {}{}", channel.prefix(), channel.name)
                }
            };

            let is_selected = i == app.selected_sidebar_idx;
            let is_active = match item {
                SidebarItem::DmChannel(dm) => Some(dm.id.clone()) == app.selected_channel,
                SidebarItem::Channel(channel) => Some(channel.id.clone()) == app.selected_channel,
                _ => false,
            };

            let is_section = matches!(item, SidebarItem::DmSection | SidebarItem::ServerSection);

            let style = if is_section {
                Style::default()
                    .fg(theme.get_color("base0D"))
                    .add_modifier(Modifier::BOLD)
            } else if is_selected && app.mode == AppMode::Sidebar {
                Style::default()
                    .fg(theme.get_color("base0A"))
                    .add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default().fg(theme.get_color("base0B"))
            } else {
                Style::default().fg(theme.get_color("base05"))
            };

            ListItem::new(text).style(style)
        })
        .collect();

    let border_style = if app.mode == AppMode::Sidebar {
        Style::default().fg(theme.get_color("base0A"))
    } else {
        Style::default().fg(theme.get_color("base03"))
    };

    let list = List::new(list_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Navigation")
            .border_style(border_style),
    );

    f.render_widget(list, area);
}
