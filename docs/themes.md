---
layout: default
title: Themes
permalink: /configuration/themes
---

## Overview

remycord uses Base16 color schemes for theming, allowing you to match your terminal's aesthetic perfectly.

## Theme Location

Themes are stored as YAML files in:

- **macOS**: `~/Library/Application Support/remycord/themes/`
- **Linux**: `~/.config/remycord/themes/`
- **Windows**: `%APPDATA%\remycord\themes\`

## Built-in Themes

remycord includes the **Oxocarbon Dark** theme by default.

## Theme Structure

A theme file (`theme-name.yaml`) contains 16 Base16 colors plus metadata:

```yaml
name: "Theme Name"
author: "Author Name"
base00: "#161616"  # Background
base01: "#262626"  # Lighter Background
base02: "#393939"  # Selection Background
base03: "#525252"  # Comments, Invisibles
base04: "#dde1e6"  # Dark Foreground
base05: "#f2f4f8"  # Default Foreground
base06: "#ffffff"  # Light Foreground
base07: "#08bdba"  # Light Background
base08: "#3ddbd9"  # Red (Errors, Deletions)
base09: "#78a9ff"  # Orange (Warnings)
base0A: "#ee5396"  # Yellow (Search Highlight)
base0B: "#33b1ff"  # Green (Strings, Success)
base0C: "#ff7eb6"  # Cyan (Support, Regex)
base0D: "#42be65"  # Blue (Functions, Links)
base0E: "#be95ff"  # Purple (Keywords)
base0F: "#82cfff"  # Magenta (Deprecated)
```

## Color Usage in remycord

| Base | Usage |
|------|-------|
| `base00` | Main background |
| `base01` | Sidebar background |
| `base02` | Selection highlight |
| `base03` | Borders, disabled text |
| `base04` | Secondary text |
| `base05` | Primary text |
| `base06` | Emphasized text |
| `base07` | Alternative highlight |
| `base08` | Errors, important warnings |
| `base09` | Warnings, notifications |
| `base0A` | Active/focused elements |
| `base0B` | Success, active channel |
| `base0C` | Links, mentions |
| `base0D` | Headings, sections |
| `base0E` | Usernames, highlights |
| `base0F` | Special elements |

## Installing Themes

### From Base16

1. Browse themes at [base16-project.github.io](https://base16-project.github.io/)
2. Download or copy the YAML theme file
3. Save to `~/.config/remycord/themes/theme-name.yaml`
4. Select in settings or config file

### Creating Custom Themes

Create a new YAML file in the themes directory:

```yaml
name: "My Custom Theme"
author: "Your Name"
base00: "#1e1e1e"
base01: "#2d2d2d"
base02: "#3a3a3a"
base03: "#4a4a4a"
base04: "#b0b0b0"
base05: "#d4d4d4"
base06: "#e8e8e8"
base07: "#ffffff"
base08: "#f44747"
base09: "#d19a66"
base0A: "#e5c07b"
base0B: "#98c379"
base0C: "#56b6c2"
base0D: "#61afef"
base0E: "#c678dd"
base0F: "#be5046"
```

## Switching Themes

### Using Settings Menu

1. Press `s` to open settings
2. Navigate to "Theme" setting
3. Press `Enter` to cycle through available themes
4. Changes apply immediately

### Via Configuration File

Edit `~/.config/remycord/config.toml`:

```toml
theme_name = "gruvbox-dark"
```

Restart remycord to apply.

### Via Command Line

```bash
REMYCORD_THEME=theme-name remycord
```

## Popular Themes

### Gruvbox Dark

```yaml
name: "Gruvbox Dark"
author: "Pavel Pertsev"
base00: "#282828"
base01: "#3c3836"
base02: "#504945"
base03: "#665c54"
base04: "#bdae93"
base05: "#d5c4a1"
base06: "#ebdbb2"
base07: "#fbf1c7"
base08: "#fb4934"
base09: "#fe8019"
base0A: "#fabd2f"
base0B: "#b8bb26"
base0C: "#8ec07c"
base0D: "#83a598"
base0E: "#d3869b"
base0F: "#d65d0e"
```

### Nord

```yaml
name: "Nord"
author: "arcticicestudio"
base00: "#2E3440"
base01: "#3B4252"
base02: "#434C5E"
base03: "#4C566A"
base04: "#D8DEE9"
base05: "#E5E9F0"
base06: "#ECEFF4"
base07: "#8FBCBB"
base08: "#BF616A"
base09: "#D08770"
base0A: "#EBCB8B"
base0B: "#A3BE8C"
base0C: "#88C0D0"
base0D: "#81A1C1"
base0E: "#B48EAD"
base0F: "#5E81AC"
```

### Dracula

```yaml
name: "Dracula"
author: "Mike Barkmin"
base00: "#282a36"
base01: "#44475a"
base02: "#6272a4"
base03: "#6272a4"
base04: "#f8f8f2"
base05: "#f8f8f2"
base06: "#f8f8f2"
base07: "#f8f8f2"
base08: "#ff5555"
base09: "#ffb86c"
base0A: "#f1fa8c"
base0B: "#50fa7b"
base0C: "#8be9fd"
base0D: "#bd93f9"
base0E: "#ff79c6"
base0F: "#ff79c6"
```

### Solarized Dark

```yaml
name: "Solarized Dark"
author: "Ethan Schoonover"
base00: "#002b36"
base01: "#073642"
base02: "#586e75"
base03: "#657b83"
base04: "#839496"
base05: "#93a1a1"
base06: "#eee8d5"
base07: "#fdf6e3"
base08: "#dc322f"
base09: "#cb4b16"
base0A: "#b58900"
base0B: "#859900"
base0C: "#2aa198"
base0D: "#268bd2"
base0E: "#6c71c4"
base0F: "#d33682"
```

## Theme Development

### Testing Your Theme

1. Save your theme file
2. Select it in remycord
3. Navigate through different views
4. Check readability and contrast

### Best Practices

1. **Maintain contrast** - Ensure text is readable on backgrounds
2. **Test both modes** - Check sidebar, messages, input, and settings
3. **Consider accessibility** - Use sufficient color contrast ratios
4. **Follow Base16 semantics** - Use colors for their intended purposes
5. **Test with content** - View actual messages and UI elements

### Color Contrast Tools

- [Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [Colorable](https://colorable.jxnblk.com/)
- [Accessible Colors](https://accessible-colors.com/)

## Sharing Themes

Create a pull request to add your theme to the official repository:

1. Fork the remycord repository
2. Add your theme to `themes/`
3. Test thoroughly
4. Submit a pull request

Include:
- Theme file
- Screenshots
- Color palette preview
- Attribution if based on another theme

## Troubleshooting

### Theme Not Appearing

- Check file is in correct directory
- Verify YAML syntax is valid
- Ensure filename ends with `.yaml`
- Restart remycord

### Colors Look Wrong

- Check terminal color settings
- Verify hex color values
- Test in different terminals
- Try a different theme

### Theme Not Loading

Check the theme file for:
- Valid YAML syntax
- All 16 base colors present
- Proper hex color format (`#RRGGBB`)
- No typos in color names

## Light Themes

While remycord primarily targets dark themes, light themes are supported:

```yaml
name: "Solarized Light"
author: "Ethan Schoonover"
base00: "#fdf6e3"  # Light background
base01: "#eee8d5"
# ... (invert dark/light values)
```

Most Base16 themes have both dark and light variants.
