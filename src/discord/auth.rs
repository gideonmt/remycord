use anyhow::{Context, Result};
use std::process::Command;

pub struct DiscordAuth;

impl DiscordAuth {
    pub fn get_token() -> Result<String> {
        #[cfg(target_os = "windows")]
        {
            Self::get_token_windows()
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::get_token_macos()
        }
        
        #[cfg(target_os = "linux")]
        {
            Self::get_token_linux()
        }
    }
   
    // idk if this works, cant test it
    #[cfg(target_os = "windows")]
    fn get_token_windows() -> Result<String> {
        let output = Command::new("cmdkey")
            .args(["/list:remycord"])
            .output()
            .context("Failed to execute cmdkey")?;
        
        if !output.status.success() {
            anyhow::bail!("Token not found in Windows Credential Manager. Please run:\ncmdkey /add:remycord /user:token /pass:YOUR_DISCORD_TOKEN");
        }
        
        let ps_output = Command::new("powershell")
            .args([
                "-Command",
                "$cred = Get-StoredCredential -Target remycord; if ($cred) { $cred.GetNetworkCredential().Password } else { exit 1 }"
            ])
            .output()
            .context("Failed to retrieve token from Windows Credential Manager")?;
        
        if !ps_output.status.success() {
            anyhow::bail!("Token not found. Please run:\ncmdkey /add:remycord /user:token /pass:YOUR_DISCORD_TOKEN");
        }
        
        let token = String::from_utf8(ps_output.stdout)
            .context("Invalid UTF-8 in token")?
            .trim()
            .to_string();
        
        if token.is_empty() {
            anyhow::bail!("Token is empty");
        }
        
        Ok(token)
    }
    
    #[cfg(target_os = "macos")]
    fn get_token_macos() -> Result<String> {
        let output = Command::new("security")
            .args([
                "find-generic-password",
                "-s", "remycord",
                "-a", "token",
                "-w"
            ])
            .output()
            .context("Failed to execute security command")?;
        
        if !output.status.success() {
            anyhow::bail!("Token not found in macOS Keychain. Please run:\nsecurity add-generic-password -s remycord -a token -w \"YOUR_DISCORD_TOKEN\"");
        }
        
        let token = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in token")?
            .trim()
            .to_string();
        
        if token.is_empty() {
            anyhow::bail!("Token is empty");
        }
        
        Ok(token)
    }
    
    #[cfg(target_os = "linux")]
    fn get_token_linux() -> Result<String> {
        let output = Command::new("secret-tool")
            .args([
                "lookup",
                "service", "remycord",
                "username", "token"
            ])
            .output()
            .context("Failed to execute secret-tool. Make sure gnome-keyring is installed and running.")?;
        
        if !output.status.success() {
            anyhow::bail!(
                "Token not found in GNOME Keyring. Please run:\n\
                eval $(gnome-keyring-daemon --start)\n\
                export $(gnome-keyring-daemon --start)\n\
                secret-tool store --label=\"Discord Token\" service remycord username token"
            );
        }
        
        let token = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in token")?
            .trim()
            .to_string();
        
        if token.is_empty() {
            anyhow::bail!("Token is empty");
        }
        
        Ok(token)
    }
}
