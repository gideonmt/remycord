---
layout: default
title: Configuration
permalink: /configuration/
---

# Configuration

Configuration file is located at `~/.config/remycord/config.toml` (Linux/macOS) or `%APPDATA%\remycord\config.toml` (Windows).

## General Settings

```toml
[general]
username = "You"
file_manager = "fzf"
show_timestamps = true
show_typing_indicators = true
message_scroll_speed = 1
max_input_lines = 8
```

- `username` - Display name for your messages
- `file_manager` - File picker for attachments (`fzf` or `lf`)
- `show_timestamps` - Show message timestamps
- `show_typing_indicators` - Display typing indicators
- `message_scroll_speed` - Lines per scroll (1-5)
- `max_input_lines` - Maximum input box height (4-12)

## Image Settings

This is still a **work in progress** feature.

```toml
[images]
enabled = true
render_avatars = true
render_emojis = true
render_stickers = true
render_attachments = true
render_server_icons = true
max_image_width = 30
max_image_height = 15
```

Image support requires Kitty terminal or compatible terminal emulator.

## Themes

```toml
theme_name = "oxocarbon-dark"
```

Themes are Base16 YAML files stored in `~/.config/remycord/themes/`.

See (themes)[/themes] for more. 

## Keybindings

```toml
[keybinds]
quit = { key = "q", modifiers = [] }
settings = { key = "s", modifiers = [] }
up = { key = "k", modifiers = [] }
down = { key = "j", modifiers = [] }
select = { key = "Enter", modifiers = [] }
back = { key = "Esc", modifiers = [] }
input_mode = { key = "i", modifiers = [] }
attach_file = { key = "a", modifiers = [] }
scroll_up = { key = "k", modifiers = [] }
scroll_down = { key = "j", modifiers = [] }
send_message = { key = "Enter", modifiers = [] }
cancel_input = { key = "Esc", modifiers = [] }
cursor_left = { key = "Left", modifiers = [] }
cursor_right = { key = "Right", modifiers = [] }
cursor_start = { key = "a", modifiers = ["Ctrl"] }
cursor_end = { key = "e", modifiers = ["Ctrl"] }
```

Available modifiers: `"Ctrl"`, `"Alt"`, `"Shift"`

### Custom Keybindings Example

```toml
[keybinds]
quit = { key = "q", modifiers = ["Ctrl"] }
settings = { key = ",", modifiers = [] }
send_message = { key = "Enter", modifiers = ["Ctrl"] }
```

## Editing Configuration

You can edit settings in two ways:

1. **Settings menu** - Press `s` and navigate with arrow keys
2. **Manual edit** - Edit `config.toml` directly and restart

## Resetting Configuration

Delete the config file to reset to defaults:

```bash
rm ~/.config/remycord/config.toml
```
