use std::collections::HashMap;
use std::sync::Arc;

use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::{Terminal, TerminalOptions, Viewport};
use russh::keys::ssh_key::PublicKey;
use russh::server::*;
use russh::{Channel, ChannelId, Pty};
use tokio::sync::Mutex;

use crate::app::{App, AppMode};
use crate::game::Direction;
use crate::terminal::TerminalHandle;
use crate::ui;

type SshTerminal = Terminal<CrosstermBackend<TerminalHandle>>;

/// Per-client state: a ratatui terminal and the app model.
type ClientState = (SshTerminal, App);

/// SSH server that serves the portfolio TUI to each connected client.
#[derive(Clone)]
pub struct AppServer {
    clients: Arc<Mutex<HashMap<usize, ClientState>>>,
    id: usize,
}

impl AppServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            id: 0,
        }
    }

    /// Start the SSH server on the given address.
    pub async fn run(
        &mut self,
        config: Arc<russh::server::Config>,
        addr: (&str, u16),
    ) -> Result<(), anyhow::Error> {
        self.run_on_address(config, addr).await?;
        Ok(())
    }

    /// Re-render the TUI for a specific client.
    async fn render_client(&self, id: usize) {
        let mut clients = self.clients.lock().await;
        if let Some((terminal, app)) = clients.get_mut(&id) {
            let _ = terminal.draw(|f| {
                ui::render(app, f);
            });
        }
    }

    /// Spawn the continuous animation ticker for a client.
    fn spawn_animation_ticker(&self, id: usize) {
        let clients = self.clients.clone();
        tokio::spawn(async move {
            let tick_ms: u64 = 100; // Drifting speed

            loop {
                tokio::time::sleep(std::time::Duration::from_millis(tick_ms)).await;

                let mut guard = clients.lock().await;
                let client_exists = if let Some((terminal, app)) = guard.get_mut(&id) {
                    // advance_intro handles both the intro phases and the continuous animation_tick
                    app.advance_intro(4);

                    // Advance typewriter output if active
                    if app.mode == AppMode::TypewriterOutput {
                        app.advance_typewriter(6);
                    }

                    // Advance snake game if active
                    if app.mode == AppMode::SnakeGame {
                        app.snake_tick_counter += 1;
                        if app.snake_tick_counter >= app.snake_speed {
                            app.snake_tick_counter = 0;
                            if let Some(ref mut game) = app.snake_game {
                                game.tick();
                            }
                        }
                    }

                    let _ = terminal.draw(|f| {
                        ui::render(app, f);
                    });
                    true
                } else {
                    false
                };

                if !client_exists {
                    break;
                }
            }
        });
    }

    /// Send custom MOTD (Message of the Day)
    async fn send_motd(&self, session: &mut Session, channel: Channel<Msg>) -> Result<(), anyhow::Error> {
        let industrial_banner = r#"
    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║  ██████╗ ██████╗  ██████╗ ██╗  ██╗██╗   ██╗███████╗██████╗  ║
    ║  ██╔══██╗██╔══██╗██╔═══██╗╚██╗██╔╝╚██╗ ██╔╝██╔════╝██╔══██╗ ║
    ║  ██████╔╝██████╔╝██║   ██║ ╚███╔╝  ╚████╔╝ █████╗  ██████╔╝ ║
    ║  ██╔═══╝ ██╔══██╗██║   ██║ ██╔██╗   ╚██╔╝  ██╔══╝  ██╔══██╗ ║
    ║  ██║     ██║  ██║╚██████╔╝██╔╝ ██╗   ██║   ███████╗██║  ██║ ║
    ║  ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝ ║
    ║                                                              ║
    ║              Redwan's Industrial Terminal                    ║
    ║              Authorized Access Only                          ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝

    System Status: ONLINE
    Connection Type: SSH
    Security Level: PUBLIC ACCESS
    Last Login: {}
    "#;

        let _timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        let motd = format!(
            "{}\r\n{}Welcome to Redwan's SSH Portfolio{}\r\n{}\r\n",
            industrial_banner,
            "\x1b[1;36m", // Cyan bold
            "\x1b[0m",    // Reset
            "=".repeat(40)
        );
        
        session.data(channel.id(), motd.into_bytes().into())?;
        Ok(())
    }

    /// Show exit animation before disconnecting
    async fn show_exit_animation(&self, session: &mut Session, channel_id: ChannelId) -> Result<(), anyhow::Error> {
        let shutdown_sequence = r#"
    ╔══════════════════════════════════════════════════════════════╗
    ║                                                              ║
    ║  ██████╗ ███████╗███╗   ██╗██████╗  ██████╗ ██╗   ██╗       ║
    ║  ██╔══██╗██╔════╝████╗  ██║██╔══██╗██╔═══██╗╚██╗ ██╔╝       ║
    ║  ██████╔╝█████╗  ██╔██╗ ██║██║  ██║██║   ██║ ╚████╔╝        ║
    ║  ██╔══██╗██╔══╝  ██║╚██╗██║██║  ██║██║   ██║  ╚██╔╝         ║
    ║  ██║  ██║███████╗██║ ╚████║██████╔╝╚██████╔╝   ██║          ║
    ║  ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝╚═════╝  ╚═════╝    ╚═╝          ║
    ║                                                              ║
    ║              System Shutdown Initiated                       ║
    ║              Connection Terminating                          ║
    ║                                                              ║
    ╚══════════════════════════════════════════════════════════════╝

    Logging out...
    Session ended.
    "#;

        let exit_msg = format!(
            "{}\r\n{}Disconnecting from Redwan's SSH Portfolio{}\r\n{}\r\n",
            shutdown_sequence,
            "\x1b[1;31m", // Red bold for shutdown
            "\x1b[0m",    // Reset
            "=".repeat(40)
        );
        
        session.data(channel_id, exit_msg.into_bytes().into())?;
        
        // Small delay to let the animation display
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        
        Ok(())
    }
}

impl Server for AppServer {
    type Handler = Self;

    fn new_client(&mut self, peer: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        log::info!(
            "New client connection (id={}) from {:?}",
            self.id,
            peer
        );
        s
    }
}

impl Handler for AppServer {
    type Error = anyhow::Error;

    /// Accept all connections without authentication.
    async fn auth_none(&mut self, _user: &str) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    /// Also accept any public key (fallback for clients that try pubkey first).
    async fn auth_publickey(
        &mut self,
        _user: &str,
        _key: &PublicKey,
    ) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    /// Client opens an SSH session channel — create the terminal + app.
    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        let terminal_handle =
            TerminalHandle::start(session.handle(), channel.id()).await;

        let backend = CrosstermBackend::new(terminal_handle);
        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::default()),
        };
        let terminal = Terminal::with_options(backend, options)?;
        let app = App::new();

        self.clients.lock().await.insert(self.id, (terminal, app));

        // Send custom MOTD (Message of the Day)
        self.send_motd(session, channel).await?;

        Ok(true)
    }

    /// Client requests a PTY — capture the terminal dimensions, do the
    /// initial render, and start the intro animation.
    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _term: &str,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _modes: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        {
            let mut clients = self.clients.lock().await;
            if let Some((terminal, app)) = clients.get_mut(&self.id) {
                terminal.resize(rect)?;
                app.resize_game(rect.width, rect.height);
            }
        }

        // Initial render (will show the intro animation first frame)
        self.render_client(self.id).await;

        // Start the animation ticker
        self.spawn_animation_ticker(self.id);

        session.channel_success(channel)?;
        Ok(())
    }

    /// Client resized their terminal window.
    async fn window_change_request(
        &mut self,
        _channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        {
            let mut clients = self.clients.lock().await;
            if let Some((terminal, app)) = clients.get_mut(&self.id) {
                terminal.resize(rect)?;
                app.resize_game(rect.width, rect.height);
            }
        }

        self.render_client(self.id).await;
        Ok(())
    }

    /// Client sent data (keypresses as raw bytes).
    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let mut should_quit = false;
        let mut needs_render = false;

        {
            let mut clients = self.clients.lock().await;
            if let Some((terminal, app)) = clients.get_mut(&self.id) {
                // If intro is still playing, any keypress skips it
                if !app.intro_done() {
                    app.skip_intro();
                    let _ = terminal.draw(|f| {
                        ui::render(app, f);
                    });
                } else {
                    // Get viewport height for scroll calculations
                    let viewport_h = terminal.size().map(|s| s.height).unwrap_or(24);
                    let content_h = viewport_h.saturating_sub(14) as usize;

                    match app.mode.clone() {
                        AppMode::Normal => {
                            let on_creations_tab = app.tab == crate::app::Tab::Creations;
                            match data {
                                // 'q' or Ctrl-C — quit
                                b"q" | b"Q" | b"\x03" => {
                                    self.show_exit_animation(session, channel).await?;
                                    app.quit();
                                    should_quit = true;
                                }
                                // ':' or '/' — enter command mode
                                b":" | b"/" => {
                                    app.enter_command_mode();
                                    needs_render = true;
                                }
                                // Right arrow, Tab, or vim 'l' — next tab
                                b"\x1b[C" | b"\t" | b"l" => {
                                    app.next_tab();
                                    needs_render = true;
                                }
                                // Left arrow, Shift-Tab, or vim 'h' — prev tab
                                b"\x1b[D" | b"\x1b[Z" | b"h" => {
                                    app.prev_tab();
                                    needs_render = true;
                                }
                                // Up arrow or vim 'k' — scroll up / select previous project
                                b"\x1b[A" | b"k" => {
                                    if on_creations_tab {
                                        app.select_prev_project();
                                    } else {
                                        app.scroll_up();
                                    }
                                    needs_render = true;
                                }
                                // Down arrow or vim 'j' — scroll down / select next project
                                b"\x1b[B" | b"j" => {
                                    if on_creations_tab {
                                        app.select_next_project();
                                    } else {
                                        let total = app.content_line_count();
                                        app.scroll_down(total, content_h);
                                    }
                                    needs_render = true;
                                }
                                // vim 'g' — scroll to top / select first project
                                b"g" => {
                                    if on_creations_tab {
                                        app.select_first_project();
                                    } else {
                                        app.scroll_offset = 0;
                                    }
                                    needs_render = true;
                                }
                                // vim 'G' — scroll to bottom / select last project
                                b"G" => {
                                    if on_creations_tab {
                                        app.select_last_project();
                                    } else {
                                        let total = app.content_line_count();
                                        if total > content_h {
                                            app.scroll_offset = total - content_h;
                                        }
                                    }
                                    needs_render = true;
                                }
                                // '1' .. '3' — jump to tab directly
                                b"1" => {
                                    app.go_to_tab(0);
                                    needs_render = true;
                                }
                                b"2" => {
                                    app.go_to_tab(1);
                                    needs_render = true;
                                }
                                b"3" => {
                                    app.go_to_tab(2);
                                    needs_render = true;
                                }
                                _ => {
                                    // Ignore unknown input
                                }
                            }
                        }

                        AppMode::Command => {
                            match data {
                                // Escape — exit command mode
                                b"\x1b" => {
                                    app.exit_command_mode();
                                    needs_render = true;
                                }
                                // Ctrl-C — exit command mode
                                b"\x03" => {
                                    app.exit_command_mode();
                                    needs_render = true;
                                }
                                // Enter — execute command
                                b"\r" | b"\n" => {
                                    app.execute_command();
                                    needs_render = true;
                                }
                                // Backspace / Delete
                                b"\x7f" | b"\x08" => {
                                    app.command_backspace();
                                    needs_render = true;
                                }
                                // Tab — auto-complete
                                b"\t" => {
                                    app.tab_complete();
                                    needs_render = true;
                                }
                                // Up arrow — history up
                                b"\x1b[A" => {
                                    app.command_history_up();
                                    needs_render = true;
                                }
                                // Down arrow — history down
                                b"\x1b[B" => {
                                    app.command_history_down();
                                    needs_render = true;
                                }
                                // Left arrow — move cursor left
                                b"\x1b[D" => {
                                    app.command_move_left();
                                    needs_render = true;
                                }
                                // Right arrow — move cursor right
                                b"\x1b[C" => {
                                    app.command_move_right();
                                    needs_render = true;
                                }
                                _ => {
                                    // Regular character input
                                    if let Ok(s) = std::str::from_utf8(data) {
                                        for c in s.chars() {
                                            if c.is_ascii_graphic() || c == ' ' {
                                                app.command_insert_char(c);
                                            }
                                        }
                                        needs_render = true;
                                    }
                                }
                            }
                        }

                        AppMode::TypewriterOutput => {
                            match data {
                                // Escape, 'q', or Enter when done — back to command mode
                                b"\x1b" | b"q" | b"Q" => {
                                    app.exit_typewriter();
                                    app.enter_command_mode();
                                    needs_render = true;
                                }
                                b"\r" | b"\n" => {
                                    if app.typewriter_done() {
                                        app.exit_typewriter();
                                        app.enter_command_mode();
                                        needs_render = true;
                                    } else {
                                        // Skip to end of typewriter
                                        app.output_chars_shown = app.output_chars_total;
                                        needs_render = true;
                                    }
                                }
                                // Any key skips typewriter animation
                                b" " => {
                                    app.output_chars_shown = app.output_chars_total;
                                    needs_render = true;
                                }
                                _ => {
                                    // Skip typewriter on any key
                                    if !app.typewriter_done() {
                                        app.output_chars_shown = app.output_chars_total;
                                        needs_render = true;
                                    }
                                }
                            }
                        }

                        AppMode::SnakeGame => {
                            match data {
                                // Escape or 'q' — exit game
                                b"\x1b" | b"q" | b"Q" => {
                                    app.exit_snake();
                                    needs_render = true;
                                }
                                // Arrow keys and WASD for direction
                                b"\x1b[A" | b"w" | b"W" => {
                                    if let Some(ref mut game) = app.snake_game {
                                        game.change_direction(Direction::Up);
                                    }
                                    needs_render = true;
                                }
                                b"\x1b[B" | b"s" | b"S" => {
                                    if let Some(ref mut game) = app.snake_game {
                                        game.change_direction(Direction::Down);
                                    }
                                    needs_render = true;
                                }
                                b"\x1b[D" | b"a" | b"A" => {
                                    if let Some(ref mut game) = app.snake_game {
                                        game.change_direction(Direction::Left);
                                    }
                                    needs_render = true;
                                }
                                b"\x1b[C" | b"d" | b"D" => {
                                    if let Some(ref mut game) = app.snake_game {
                                        game.change_direction(Direction::Right);
                                    }
                                    needs_render = true;
                                }
                                // 'r' to restart if game over
                                b"r" | b"R" => {
                                    if let Some(ref game) = app.snake_game {
                                        if game.game_over {
                                            let w = game.width;
                                            let h = game.height;
                                            app.snake_game = Some(crate::game::SnakeGame::new(w, h));
                                            app.snake_tick_counter = 0;
                                        }
                                    }
                                    needs_render = true;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        if should_quit {
            self.clients.lock().await.remove(&self.id);
            session.close(channel)?;
        } else if needs_render {
            self.render_client(self.id).await;
        }

        Ok(())
    }
}

impl Drop for AppServer {
    fn drop(&mut self) {
        let id = self.id;
        let clients = self.clients.clone();
        tokio::spawn(async move {
            clients.lock().await.remove(&id);
        });
    }
}
