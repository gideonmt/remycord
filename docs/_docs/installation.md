---
layout: doc
title: Getting Started
---

## First Launch

After [installing]({{ '/docs/installation/' | relative_url }}) remycord and [setting up your token]({{ '/docs/token-setup/' | relative_url }}), launch the application:

```bash
remycord
```

You should see the main interface with:
- **Sidebar** on the left showing servers and DMs
- **Help panel** on the right with keybinding information

## Interface Overview

### Sidebar (Left Panel)

The sidebar contains:
- **Direct Messages** section (collapsible)
  - Your DM conversations
- **Servers** section
  - Your Discord servers
  - Expandable to show channels

### Main Area (Right Panel)

Depending on your current mode:
- **Help** - Shows available keybindings
- **Messages** - Shows channel messages when a channel is selected
- **Input** - Message composition area
- **Settings** - Configuration interface

## Basic Navigation

### Selecting a Channel

1. Use `j`/`k` (or arrow keys) to move up and down in the sidebar
2. Press `Enter` to:
   - Expand/collapse the DM section
   - Expand/collapse a server
   - Select a channel

### Reading Messages

Once in a channel:
- Use `j`/`k` to scroll through messages
- Press `i` to enter input mode
- Press `Esc` or `q` to go back to the sidebar

### Sending Messages

1. Press `i` to enter input mode
2. Type your message
3. Press `Enter` to send
4. Press `Esc` to cancel

## Common Tasks

### Sending a DM

1. Navigate to the **Direct Messages** section
2. Press `Enter` to expand if collapsed
3. Select a contact with `j`/`k` and press `Enter`
4. Press `i` to start typing
5. Type your message and press `Enter`

### Joining a Channel

1. Navigate to a server in the sidebar
2. Press `Enter` to expand the server
3. Select a channel with `j`/`k`
4. Press `Enter` to open the channel

### Attaching Files

1. While viewing messages, press `a`
2. Use the file picker (fzf or lf) to select a file
3. Press `i` to enter input mode
4. Optionally add a message
5. Press `Enter` to send

### Accessing Settings

Press `s` from anywhere to open the settings panel.

## Understanding Modes

remycord has several modes:

### Sidebar Mode (Default)
- Navigate between servers and channels
- Expand/collapse sections
- Select channels to view

### Messages Mode
- Read messages in the selected channel
- Scroll through message history
- Enter input mode or attach files

### Input Mode
- Compose messages
- Navigate with cursor keys
- Send or cancel messages

### Settings Mode
- Configure remycord
- Customize keybindings
- Change themes

## Tips for New Users

1. **Start with DMs** - They're simpler than navigating servers
2. **Learn the basics** - Focus on `j`, `k`, `Enter`, `i`, and `Esc` first
3. **Use the help screen** - Press `q` from messages to see keybindings
4. **Customize later** - Get comfortable before changing keybindings
5. **Enable images** - If using Kitty terminal, enable image support in settings

## Default Keybindings Quick Reference

| Key | Action |
|-----|--------|
| `j` | Move down |
| `k` | Move up |
| `Enter` | Select/Expand |
| `q` | Quit/Back |
| `i` | Enter input mode |
| `a` | Attach file |
| `s` | Open settings |
| `Esc` | Go back/Cancel |

## Next Steps

- [Configure your settings]({{ '/docs/configuration/' | relative_url }})
- [Customize keybindings]({{ '/docs/keybindings/' | relative_url }})
- [Set up themes]({{ '/docs/themes/' | relative_url }})
- [Enable image support]({{ '/docs/images/' | relative_url }})

## Getting Help

If you encounter issues:
1. Check the [troubleshooting section]({{ '/docs/installation/' | relative_url }}#troubleshooting)
2. Open an issue on [GitHub](https://github.com/yourusername/remycord/issues)
3. Join our [discussions](https://github.com/yourusername/remycord/discussions)
