---
layout: doc
title: Configuration
---

## Configuration File Location

remycord stores its configuration in:

- **macOS**: `~/Library/Application Support/remycord/config.toml`
- **Linux**: `~/.config/remycord/config.toml`
- **Windows**: `%APPDATA%\remycord\config.toml`

The configuration file is automatically created on first launch with sensible defaults.

## Configuration Structure

The configuration file is in TOML format with the following sections:

```toml
[general]
username = "You"
file_manager = "fzf"
show_timestamps = true
show_typing_indicators = true
message_scroll_speed = 1
max_input_lines = 8

[images]
enabled = true
render_avatars = true
render_emojis = true
render_stickers = true
render_attachments = true
render_server_icons = true
max_image_width = 30
max_image_height = 15

theme_name = "oxocarbon-dark"

[keybinds]
# See keybindings documentation
```

## General Settings

### username
- **Type**: String
- **Default**: `"You"`
- **Description**: Display name for your messages

### file_manager
- **Type**: String
- **Default**: `"fzf"`
- **Options**: `"fzf"`, `"lf"`
- **Description**: File picker for attachments

### show_timestamps
- **Type**: Boolean
- **Default**: `true`
- **Description**: Show message timestamps

### show_typing_indicators
- **Type**: Boolean
- **Default**: `true`
- **Description**: Display when others are typing

### message_scroll_speed
- **Type**: Integer
- **Default**: `1`
- **Range**: 1-5
- **Description**: Lines scrolled per keypress

### max_input_lines
- **Type**: Integer
- **Default**: `8`
- **Range**: 4-12
- **Description**: Maximum lines for input box

## Image Settings

Image support requires a compatible terminal (Kitty recommended).

### enabled
- **Type**: Boolean
- **Default**: Auto-detected (true if Kitty)
- **Description**: Enable image rendering

### render_avatars
- **Type**: Boolean
- **Default**: `true`
- **Description**: Show user avatars

### render_emojis
- **Type**: Boolean
- **Default**: `true`
- **Description**: Render custom Discord emojis

### render_stickers
- **Type**: Boolean
- **Default**: `true`
- **Description**: Display Discord stickers

### render_attachments
- **Type**: Boolean
- **Default**: `true`
- **Description**: Show image attachments inline

### render_server_icons
- **Type**: Boolean
- **Default**: `true`
- **Description**: Display server icons in sidebar

### max_image_width
- **Type**: Integer
- **Default**: `30`
- **Description**: Maximum image width in columns

### max_image_height
- **Type**: Integer
- **Default**: `15`
- **Description**: Maximum image height in rows

## Theme Settings

### theme_name
- **Type**: String
- **Default**: `"oxocarbon-dark"`
- **Description**: Active theme name

Themes are stored in `~/.config/remycord/themes/` as YAML files.

See [Themes documentation]({{ '/docs/themes/' | relative_url }}) for more details.

## Editing Configuration

### Using Settings Menu

The easiest way to change settings:

1. Press `s` to open settings
2. Navigate with `↑`/`↓`
3. Press `Enter` to toggle/cycle values
4. Press `Esc` to save and exit

### Manual Editing

You can also edit the config file directly:

```bash
# macOS/Linux
vim ~/.config/remycord/config.toml

# Windows
notepad %APPDATA%\remycord\config.toml
```

Changes take effect after restarting remycord.

## Configuration Examples

### Minimal Setup
```toml
[general]
username = "Me"
file_manager = "fzf"
show_timestamps = false
show_typing_indicators = false

[images]
enabled = false

theme_name = "oxocarbon-dark"
```

### Power User Setup
```toml
[general]
username = "PowerUser"
file_manager = "lf"
show_timestamps = true
show_typing_indicators = true
message_scroll_speed = 3
max_input_lines = 12

[images]
enabled = true
render_avatars = true
render_emojis = true
render_stickers = true
render_attachments = true
render_server_icons = true
max_image_width = 50
max_image_height = 25

theme_name = "gruvbox-dark"
```

### Performance-Focused
```toml
[general]
username = "FastUser"
file_manager = "fzf"
show_timestamps = false
show_typing_indicators = false
message_scroll_speed = 5
max_input_lines = 4

[images]
enabled = false

theme_name = "oxocarbon-dark"
```

## Resetting Configuration

To reset to defaults:

1. Delete the config file
2. Restart remycord
3. A new config will be created

```bash
# macOS/Linux
rm ~/.config/remycord/config.toml

# Windows
del %APPDATA%\remycord\config.toml
```

## Configuration Validation

remycord validates your configuration on startup. If there are errors:

1. The error will be displayed
2. Default values will be used
3. Check your config file for syntax errors

Common issues:
- Invalid TOML syntax
- Incorrect value types
- Out-of-range values

## Environment Variables

Some settings can be overridden with environment variables:

```bash
# Force enable/disable images
REMYCORD_IMAGES=1 remycord  # Enable
REMYCORD_IMAGES=0 remycord  # Disable

# Override theme
REMYCORD_THEME=gruvbox-dark remycord

# Config file location
REMYCORD_CONFIG=/path/to/config.toml remycord
```
