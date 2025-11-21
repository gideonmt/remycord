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

use app::{App, AppMode};
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
        DiscordEvent::NewMessage(msg) => {
            if Some(&msg.channel_id) == app.selected_channel.as_ref() {
                if app.config.images.enabled && app.config.images.render_avatars {
                    let _ = app.image_renderer.load_avatar(
                        &msg.author_id,
                        msg.author_avatar.as_deref(),
                    ).await;
                }
                
                if app.config.images.enabled && app.config.images.render_attachments {
                    let max_w = app.config.images.max_image_width;
                    let max_h = app.config.images.max_image_height;
                    for attachment in msg.attachments.iter().filter(|a| a.is_image()) {
                        let _ = app.image_renderer.load_attachment(
                            &attachment.id,
                            &attachment.url,
                            max_w,
                            max_h,
                            true
                        ).await;
                    }
                }
                
                app.messages.push(msg.clone());
            }
            
            if let Some(messages) = app.message_cache.get_mut(&msg.channel_id) {
                messages.push(msg);
            }
        }
        _ => {}
    }
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: Arc<Mutex<App>>,
) -> Result<()> {
    let mut cache_check_timer = tokio::time::Instant::now();
    let mut cache_stats_timer = tokio::time::Instant::now();
    let cache_check_interval = std::time::Duration::from_secs(10);
    let cache_stats_interval = std::time::Duration::from_secs(5);
    
    loop {
        {
            let mut app = app.lock().await;
            
            if cache_check_timer.elapsed() >= cache_check_interval {
                app.check_cache_health().await;
                app.check_scheduled_cache_clear().await;
                cache_check_timer = tokio::time::Instant::now();
            }
            
            if cache_stats_timer.elapsed() >= cache_stats_interval {
                if matches!(app.mode, AppMode::Settings | AppMode::KeybindRecording(_)) {
                    app.update_cache_stats().await;
                }
                cache_stats_timer = tokio::time::Instant::now();
            }
            
            app.clear_expired_notifications();
            
            if app.loading_dms {
                let client_arc = app.discord_client.clone();
                if let Some(client_arc) = client_arc {
                    let client = client_arc.lock().await;
                    if let Ok(dms) = client.fetch_dms().await {
                        drop(client);
                        app.dms = dms;
                        let dm_count = app.dms.len();
                        let msg = format!("Loaded {} DM channel(s)", dm_count);
                        app.add_notification(Notification::success(msg));
                    } else {
                        app.add_notification(Notification::error("Failed to load DMs"));
                    }
                }
                app.loading_dms = false;
            }
            
            if app.loading_channels {
                let guilds_to_load: Vec<String> = app
                    .guilds
                    .iter()
                    .filter(|g| g.expanded && !app.channel_cache.contains_key(&g.id))
                    .map(|g| g.id.clone())
                    .collect();

                for guild_id in guilds_to_load {
                    let client_arc = app.discord_client.clone();
                    if let Some(client_arc) = client_arc {
                        let client = client_arc.lock().await;
                        
                        match client.fetch_channels(&guild_id).await {
                            Ok(channel_list) => {
                                let num_channels = channel_list.channels.len();
                                let num_categories = channel_list.categories.len();
                                
                                drop(client);
                                
                                app.channel_cache.insert(guild_id.clone(), channel_list);

                                if let Some(guild) = app.guilds.iter().find(|g| g.id == guild_id) {
                                    let guild_name = guild.name.clone();
                                    let label = if num_categories == 1 { "y" } else { "ies" };
                                    let msg = format!(
                                        "Loaded {} for {}: {} channel(s), {} categor{}",
                                        "channels",
                                        guild_name,
                                        num_channels,
                                        num_categories,
                                        label
                                    );
                                    app.add_notification(Notification::success(msg));
                                }
                            }
                            Err(e) => {
                                drop(client);
                                
                                if let Some(guild) = app.guilds.iter().find(|g| g.id == guild_id) {
                                    let guild_name = guild.name.clone();
                                    let msg = format!("Failed to load channels for {}: {}", guild_name, e);
                                    app.add_notification(Notification::error(msg));
                                }
                            }
                        }
                    }
                }
                
                app.loading_channels = false;
            }
            
            if app.loading_messages {
                if let Some(channel_id) = &app.selected_channel.clone() {
                    if !app.message_cache.contains_key(channel_id) {
                        let client_arc = app.discord_client.clone();
                        if let Some(client_arc) = client_arc {
                            let client = client_arc.lock().await;
                            
                            match client.fetch_messages(channel_id, 50).await {
                                Ok(messages) => {
                                    drop(client);
                                    
                                    app.messages = messages.clone();
                                    
                                    if app.config.images.enabled && app.config.images.render_avatars {
                                        for msg in &messages {
                                            let user_id = &msg.author_id;
                                            let avatar_hash = msg.author_avatar.as_deref();
                                            let _ = app.image_renderer.load_avatar(user_id, avatar_hash).await;
                                        }
                                    }
                                    
                                    if app.config.images.enabled && app.config.images.render_attachments {
                                        let max_w = app.config.images.max_image_width;
                                        let max_h = app.config.images.max_image_height;

                                        for msg in &messages {
                                            for attachment in msg.attachments.iter().filter(|a| a.is_image()) {
                                                let _ = app.image_renderer
                                                    .load_attachment(
                                                        &attachment.id, 
                                                        &attachment.url, 
                                                        max_w,
                                                        max_h,
                                                        true
                                                    )
                                                    .await;
                                            }
                                        }
                                    }
                                    
                                    app.message_cache.insert(channel_id.clone(), messages);
                                }
                                Err(e) => {
                                    drop(client);
                                    app.add_notification(Notification::error(
                                        format!("Failed to load messages: {}", e)
                                    ));
                                }
                            }
                        }
                    }
                    app.loading_messages = false;
                }
            }

            terminal.draw(|f| {
                ui::draw(f, &mut app);
                // Draw notifications on top
                ui::notifications::draw(f, &app, f.area());
            })?;
        }

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let mut app_lock = app.lock().await;
                
                let should_send = if app_lock.mode == AppMode::Input 
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
