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
use std::sync::Arc;
use tokio::sync::Mutex;

use app::App;
use config::load_config;
use input::handle_input;
use discord::{DiscordClient, DiscordEvent};
use models::Notification;

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config().unwrap_or_else(|e| {
        eprintln!("Warning: Could not load config: {}. Using defaults.", e);
        config::Config::default()
    });

    // Await the async get_token
    let token = match discord::token::get_token().await {
        Ok(token) => {
            println!("Found Discord token, connecting...");
            token
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
            return Ok(());
        }
    };

    let (discord_client, mut event_rx) = DiscordClient::new(token.clone()).await?;
    
    discord_client.start_gateway(token.clone()).await?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(config);
    app.set_discord_client(discord_client);
    
    let app = Arc::new(Mutex::new(app));
    let app_clone = app.clone();
    
    tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            let mut app = app_clone.lock().await;
            handle_discord_event(&mut app, event).await;
        }
    });

    let res = run_app(&mut terminal, app).await;

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

async fn handle_discord_event(app: &mut App, event: DiscordEvent) {
    match event {
        DiscordEvent::Ready(guilds) => {
            for guild in guilds {
                if !app.guilds.iter().any(|g| g.id == guild.id) {
                    app.guilds.push(guild);
                } else {
                    if let Some(g) = app.guilds.iter_mut().find(|g| g.id == guild.id) {
                        g.name = guild.name;
                    }
                }
            }
        }
        DiscordEvent::Connected(username) => {
            app.add_notification(Notification::success(format!("Connected as {}", username)));
        }
        _ => {}
    }
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: Arc<Mutex<App>>,
) -> Result<()> {
    loop {
        {
            let mut app = app.lock().await;
            
            app.clear_expired_notifications();
            
            if app.loading_dms {
                let client_arc = app.discord_client.clone();
                if let Some(client_arc) = client_arc {
                    let client = client_arc.lock().await;
                    if let Ok(dms) = client.fetch_dms().await {
                        drop(client);
                        app.dms = dms;
                    }
                }
                app.loading_dms = false;
            }
            
            if app.loading_channels {
                if let Some(guild) = app.guilds.iter().find(|g| g.expanded && !app.channel_cache.contains_key(&g.id)) {
                    let guild_id = guild.id.clone();
                    let client_arc = app.discord_client.clone();
                    if let Some(client_arc) = client_arc {
                        let client = client_arc.lock().await;
                        if let Ok(channels) = client.fetch_channels(&guild_id).await {
                            drop(client);
                            app.channel_cache.insert(guild_id, channels);
                        }
                    }
                    app.loading_channels = false;
                }
            }
            
            if app.loading_messages {
                if let Some(channel_id) = &app.selected_channel.clone() {
                    if !app.message_cache.contains_key(channel_id) {
                        let client_arc = app.discord_client.clone();
                        if let Some(client_arc) = client_arc {
                            let client = client_arc.lock().await;
                            if let Ok(messages) = client.fetch_messages(channel_id, 50).await {
                                drop(client);
                                app.messages = messages.clone();
                                app.message_cache.insert(channel_id.clone(), messages);
                            }
                        }
                    }
                    app.loading_messages = false;
                }
            }
            
            terminal.draw(|f| ui::draw(f, &app))?;
        }

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let mut app_lock = app.lock().await;
                
                let should_send = if app_lock.mode == app::AppMode::Input 
                    && app_lock.config.keybinds.send_message.matches(key.code, key.modifiers) {
                    true
                } else {
                    false
                };
                
                if should_send {
                    let channel_id = app_lock.selected_channel.clone();
                    let content = app_lock.input.clone();
                    
                    if !content.is_empty() {
                        if let Some(channel_id) = channel_id {
                            let client_arc = app_lock.discord_client.clone();
                            if let Some(client_arc) = client_arc {
                                let client = client_arc.lock().await;
                                let _ = client.send_message(&channel_id, &content).await;
                                drop(client);
                            }
                        }
                        
                        app_lock.input.clear();
                        app_lock.input_cursor = 0;
                    }
                }
                
                if handle_input(&mut app_lock, key, terminal)? {
                    return Ok(());
                }
            }
        }
    }
}
