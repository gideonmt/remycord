---
layout: doc
title: Token Setup
---

## Getting Your Discord Token

> **Warning:** Your Discord token is like a password. Never share it with anyone or commit it to version control.

### Finding Your Token

1. Open Discord in your web browser (not the app)
2. Open Developer Tools (F12 or Ctrl+Shift+I)
3. Go to the **Application** tab
4. Expand **Local Storage** → `https://discord.com`
5. Find the `token` key and copy its value

### Alternative Method (Network Tab)

1. Open Discord in your browser
2. Open Developer Tools (F12)
3. Go to the **Network** tab
4. Refresh the page (Ctrl+R or Cmd+R)
5. Look for any request to `discord.com/api`
6. Click on it and go to the **Headers** tab
7. Find `authorization` in the Request Headers
8. Copy the token value (without "Bearer " prefix if present)

## Storing Your Token Securely

remycord uses your operating system's secure credential storage to keep your token safe.

### macOS (Keychain)

```bash
security add-generic-password \
  -s remycord \
  -a token \
  -w "YOUR_DISCORD_TOKEN_HERE"
```

To verify it was stored:
```bash
security find-generic-password -s remycord -a token -w
```

To update your token:
```bash
security delete-generic-password -s remycord -a token
security add-generic-password -s remycord -a token -w "NEW_TOKEN"
```

### Linux (GNOME Keyring)

First, ensure GNOME Keyring is running:
```bash
eval $(gnome-keyring-daemon --start)
export $(gnome-keyring-daemon --start)
```

Then store your token:
```bash
secret-tool store \
  --label="Discord Token" \
  service remycord \
  username token
# You'll be prompted to enter the token
```

To retrieve your token:
```bash
secret-tool lookup service remycord username token
```

### Windows (Credential Manager)

```powershell
cmdkey /add:remycord /user:token /pass:YOUR_DISCORD_TOKEN_HERE
```

To verify:
```powershell
cmdkey /list:remycord
```

To update:
```powershell
cmdkey /delete:remycord
cmdkey /add:remycord /user:token /pass:NEW_TOKEN
```

## Testing Your Setup

After storing your token, run remycord:

```bash
remycord
```

If your token is valid, you should see:
- "Found Discord token, connecting..."
- "Successfully logged in as: YourUsername"

## Troubleshooting

### "No Discord token found"

Make sure you:
1. Stored the token using the exact service name `remycord`
2. Used the account name `token`
3. Have the correct permissions for your credential store

### "Failed to connect" or "Invalid token"

Your token might be expired or incorrect:
1. Get a fresh token from Discord
2. Delete the old token from your credential store
3. Store the new token using the commands above

### Linux: "Failed to execute secret-tool"

Install the required packages:
```bash
# Ubuntu/Debian
sudo apt install gnome-keyring libsecret-tools

# Fedora
sudo dnf install gnome-keyring libsecret

# Arch
sudo pacman -S gnome-keyring libsecret
```

Make sure D-Bus is running:
```bash
eval $(dbus-launch --auto-syntax)
```

### macOS: "security command failed"

Make sure you have permission to access the keychain:
```bash
security unlock-keychain ~/Library/Keychains/login.keychain-db
```

## Security Best Practices

1. **Never commit your token** to Git or share it publicly
2. **Use 2FA** on your Discord account for extra security
3. **Rotate your token** periodically by logging out and back into Discord
4. **Monitor your sessions** in Discord User Settings → Authorized Apps
5. **Don't use self-bots** for automation (against Discord ToS)

## Removing Your Token

If you want to remove your stored token:

**macOS:**
```bash
security delete-generic-password -s remycord -a token
```

**Linux:**
```bash
secret-tool clear service remycord username token
```

**Windows:**
```powershell
cmdkey /delete:remycord
```
