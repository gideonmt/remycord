mod app;
mod config;
mod ui;
mod models;
mod input;
mod discord;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use app::App;
use config::load_config;
use input::handle_input;

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config().unwrap_or_else(|e| {
        eprintln!("Warning: Could not load config: {}. Using defaults.", e);
        config::Config::default()
    });

    match discord::token::get_token() {
        Ok(token) => {
            println!("Found Discord token, connecting...");
            match discord::client::connect_and_verify(&token).await {
                Ok(username) => {
                    println!("Successfully logged in as: {}", username);
                }
                Err(e) => {
                    eprintln!("Failed to connect to Discord: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("No Discord token found: {}", e);
            eprintln!("To set up your token, run one of these commands:");
            eprintln!("");
            if cfg!(target_os = "windows") {
                eprintln!("  cmdkey /add:remycord /user:token /pass:YOUR_DISCORD_TOKEN");
            } else if cfg!(target_os = "macos") {
                eprintln!("  security add-generic-password -s remycord -a token -w \"YOUR_DISCORD_TOKEN\"");
            } else {
                eprintln!("  secret-tool store --label=\"Discord Token\" service remycord username token");
            }
            eprintln!("");
        }
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(config);
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if handle_input(app, key, terminal)? {
                    return Ok(());
                }
            }
        }
    }
}
