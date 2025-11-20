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
            let (text, indent_level) = match item {
                SidebarItem::DmSection => {
                    let arrow = if app.dm_section_expanded { "▼ " } else { "▶ " };
                    (format!("{}Direct Messages", arrow), 0)
                }
                SidebarItem::DmChannel(dm) => {
                    (format!("  # {}", dm.display_name()), 1)
                }
                SidebarItem::ServerSection => {
                    ("▼ Servers".to_string(), 0)
                }
                SidebarItem::Server(guild) => {
                    let arrow = if guild.expanded { "▼ " } else { "▶ " };
                    (format!("  {}{}", arrow, guild.name), 1)
                }
                SidebarItem::Category { category, expanded, .. } => {
                    let arrow = if *expanded { "▼ " } else { "▶ " };
                    (format!("    {}{}", arrow, category.name.to_uppercase()), 2)
                }
                SidebarItem::Channel { channel, .. } => {
                    let indent = if channel.parent_id.is_some() {
                        "      "
                    } else {
                        "    "
                    };
                    (format!("{}{}{}", indent, channel.prefix(), channel.name), 3)
                }
            };

            let is_selected = i == app.selected_sidebar_idx;
            let is_active = match item {
                SidebarItem::DmChannel(dm) => Some(&dm.id) == app.selected_channel.as_ref(),
                SidebarItem::Channel { channel, .. } => {
                    Some(&channel.id) == app.selected_channel.as_ref()
                }
                _ => false,
            };

            let is_section = matches!(
                item,
                SidebarItem::DmSection | SidebarItem::ServerSection
            );
            
            let is_category = matches!(item, SidebarItem::Category { .. });

            let style = if is_section {
                Style::default()
                    .fg(theme.get_color("base0D"))
                    .add_modifier(Modifier::BOLD)
            } else if is_category {
                Style::default()
                    .fg(theme.get_color("base04"))
                    .add_modifier(Modifier::BOLD)
            } else if is_selected && app.mode == AppMode::Sidebar {
                Style::default()
                    .fg(theme.get_color("base0A"))
                    .add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default()
                    .fg(theme.get_color("base0B"))
                    .add_modifier(Modifier::BOLD)
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
