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
    terminal.show_cursor()?;
    
    let file_path_result = get_file_path(&app.config.general.file_manager);
    
    terminal.hide_cursor()?;
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    enable_raw_mode()?;
    
    terminal.clear()?;
    
    if let Ok(file_path) = file_path_result {
        if !file_path.is_empty() {
            app.attached_files.push(AttachedFile::new(file_path));
        }
    }
    
    Ok(())
}

fn get_file_path(file_manager: &str) -> Result<String> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    
    if file_manager == "lf" {
        let selection_path = "/tmp/lf_selection";
        
        let _ = std::fs::remove_file(selection_path);
        
        let status = Command::new("lf")
            .arg("-selection-path")
            .arg(selection_path)
            .current_dir(&home_dir)
            .status()?;
        
        if status.success() {
            if let Ok(contents) = std::fs::read_to_string(selection_path) {
                let _ = std::fs::remove_file(selection_path);
                return Ok(contents.trim().to_string());
            }
        }
        
        Ok(String::new())
    } else {
        let find_process = Command::new("find")
            .arg(&home_dir)
            .arg("-type")
            .arg("f")
            .arg("-not")
            .arg("-path")
            .arg("*/.*")
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        
        let find_stdout = find_process.stdout
            .ok_or_else(|| anyhow::anyhow!("Failed to capture find output"))?;
        
        let fzf_output = Command::new("fzf")
            .arg("--height=40%")
            .arg("--reverse")
            .arg("--prompt=Select file: ")
            .arg("--preview=head -n 20 {}")
            .stdin(find_stdout)
            .output()?;
        
        if fzf_output.status.success() {
            Ok(String::from_utf8_lossy(&fzf_output.stdout).trim().to_string())
        } else {
            Ok(String::new())
        }
    }
}
