use anyhow::Result;
use crossterm::{
    execute,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::process::Command;

use crate::app::App;
use crate::models::AttachedFile;

pub fn pick_file(
    app: &mut App,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    
    if let Ok(file_path) = get_file_path(&app.config.general.file_manager) {
        if !file_path.is_empty() {
            app.attached_files.push(AttachedFile::new(file_path));
        }
    }
    
    enable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    
    Ok(())
}

fn get_file_path(file_manager: &str) -> Result<String> {
    if file_manager == "lf" {
        let output = Command::new("sh")
            .arg("-c")
            .arg("lf -selection-path=/tmp/lf_selection && cat /tmp/lf_selection 2>/dev/null")
            .output()?;
        
        Ok(if output.status.success() {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            String::new()
        })
    } else {
        let output = Command::new("find")
            .arg(".")
            .arg("-type")
            .arg("f")
            .stdout(std::process::Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| anyhow::anyhow!("Failed to capture find output"))?;
        
        let fzf_output = Command::new("fzf")
            .arg("--height=40%")
            .arg("--reverse")
            .stdin(output)
            .output()?;
        
        Ok(if fzf_output.status.success() {
            String::from_utf8_lossy(&fzf_output.stdout).trim().to_string()
        } else {
            String::new()
        })
    }
}
