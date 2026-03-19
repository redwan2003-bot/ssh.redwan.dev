use crate::content;
use crate::game::SnakeGame;

/// Active tab in the portfolio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Reflections,
    Creations,
    Contact,
}

impl Tab {
    pub const ALL: [Tab; 3] = [Tab::Reflections, Tab::Creations, Tab::Contact];

    pub fn label(&self) -> &'static str {
        match self {
            Tab::Reflections => "Reflections",
            Tab::Creations => "Creations",
            Tab::Contact => "Contacts",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Tab::Reflections => 0,
            Tab::Creations => 1,
            Tab::Contact => 2,
        }
    }

    pub fn from_index(i: usize) -> Option<Tab> {
        Tab::ALL.get(i).copied()
    }
}

/// The current mode of the application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppMode {
    /// Normal tab-browsing mode.
    Normal,
    /// Command prompt is active — user is typing a command.
    Command,
    /// Snake game is running.
    SnakeGame,
    /// Typewriter output is being displayed (non-interactive, just watch).
    TypewriterOutput,
}

/// Intro animation phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntroPhase {
    /// Typewriter effect — reveals chars_shown characters of the banner.
    Typing { chars_shown: usize },
    /// Brief pause after typing finishes before showing the full UI.
    Pause { ticks_remaining: u8 },
    /// Animation done — show normal UI.
    Done,
}

/// A single line of typewriter output with a style hint.
#[derive(Debug, Clone)]
pub struct OutputLine {
    pub text: String,
    pub style: OutputStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputStyle {
    System,   // [SYSTEM] cyan prefix
    Ok,       // [OK] green prefix
    Data,     // sensor data, telemetry
    Error,    // red
    Plain,    // normal white text
    Ascii,    // ASCII art (monospace, no prefix)
    Header,   // Bold section header
}

/// Application state for a single SSH client session.
pub struct App {
    pub tab: Tab,
    pub should_quit: bool,
    pub scroll_offset: usize,
    pub intro: IntroPhase,
    /// Index of the currently selected project in the flat project list (Projects tab).
    pub selected_project: usize,
    /// Continuous tick for animations (like floating stars).
    pub animation_tick: usize,

    // ── Command prompt ─────────────────────────────────────────
    pub mode: AppMode,
    /// Current text in the command input buffer.
    pub command_input: String,
    /// Cursor position within command_input.
    pub command_cursor: usize,
    /// Command history (most recent last).
    pub command_history: Vec<String>,
    /// Index into command_history when browsing with up/down (-1 = current input).
    pub history_index: Option<usize>,
    /// Saved current input when browsing history.
    pub saved_input: String,

    // ── Typewriter output ──────────────────────────────────────
    /// Lines of output to display with typewriter effect.
    pub output_lines: Vec<OutputLine>,
    /// How many characters of the output have been revealed so far.
    pub output_chars_shown: usize,
    /// Total characters in all output lines.
    pub output_chars_total: usize,
    /// Scroll offset for the output view.
    pub output_scroll: usize,

    // ── Snake game ─────────────────────────────────────────────
    pub snake_game: Option<SnakeGame>,
    /// Snake game speed: ticks between moves.
    pub snake_speed: usize,
    pub snake_tick_counter: usize,

    // ── Tab-completion ─────────────────────────────────────────
    pub tab_completions: Vec<String>,
    pub tab_completion_index: usize,
}

/// List of all available commands for tab-completion and help.
pub const COMMANDS: &[&str] = &[
    "help",
    "status",
    "heartbeat",
    "view --pcb titan-core",
    "logs --mars",
    "clear",
    "whoami",
    "theme --matrix",
    "snake",
    "exit",
];

impl App {
    pub fn new() -> Self {
        Self {
            tab: Tab::Reflections,
            should_quit: false,
            scroll_offset: 0,
            intro: IntroPhase::Typing { chars_shown: 0 },
            selected_project: 0,
            animation_tick: 0,

            mode: AppMode::Normal,
            command_input: String::new(),
            command_cursor: 0,
            command_history: Vec::new(),
            history_index: None,
            saved_input: String::new(),

            output_lines: Vec::new(),
            output_chars_shown: 0,
            output_chars_total: 0,
            output_scroll: 0,

            snake_game: None,
            snake_speed: 3,
            snake_tick_counter: 0,

            tab_completions: Vec::new(),
            tab_completion_index: 0,
        }
    }

    // ── Tab navigation ─────────────────────────────────────────

    pub fn next_tab(&mut self) {
        let idx = self.tab.index();
        let next = (idx + 1) % Tab::ALL.len();
        self.tab = Tab::ALL[next];
        self.scroll_offset = 0;
    }

    pub fn prev_tab(&mut self) {
        let idx = self.tab.index();
        let prev = if idx == 0 {
            Tab::ALL.len() - 1
        } else {
            idx - 1
        };
        self.tab = Tab::ALL[prev];
        self.scroll_offset = 0;
    }

    pub fn go_to_tab(&mut self, idx: usize) {
        if let Some(tab) = Tab::from_index(idx) {
            self.tab = tab;
            self.scroll_offset = 0;
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    // ── Mode switching ─────────────────────────────────────────

    pub fn enter_command_mode(&mut self) {
        self.mode = AppMode::Command;
        self.command_input.clear();
        self.command_cursor = 0;
        self.history_index = None;
        self.tab_completions.clear();
    }

    pub fn exit_command_mode(&mut self) {
        self.mode = AppMode::Normal;
        self.command_input.clear();
        self.command_cursor = 0;
        self.history_index = None;
        self.tab_completions.clear();
    }

    pub fn exit_typewriter(&mut self) {
        self.mode = AppMode::Normal;
        self.output_lines.clear();
        self.output_chars_shown = 0;
        self.output_chars_total = 0;
        self.output_scroll = 0;
    }

    // ── Command input handling ─────────────────────────────────

    pub fn command_insert_char(&mut self, c: char) {
        self.command_input.insert(self.command_cursor, c);
        self.command_cursor += c.len_utf8();
        self.tab_completions.clear();
    }

    pub fn command_backspace(&mut self) {
        if self.command_cursor > 0 {
            // Find the previous char boundary
            let prev = self.command_input[..self.command_cursor]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.command_input.remove(prev);
            self.command_cursor = prev;
            self.tab_completions.clear();
        }
    }

    pub fn command_move_left(&mut self) {
        if self.command_cursor > 0 {
            let prev = self.command_input[..self.command_cursor]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.command_cursor = prev;
        }
    }

    pub fn command_move_right(&mut self) {
        if self.command_cursor < self.command_input.len() {
            let next = self.command_input[self.command_cursor..]
                .char_indices()
                .nth(1)
                .map(|(i, _)| self.command_cursor + i)
                .unwrap_or(self.command_input.len());
            self.command_cursor = next;
        }
    }

    pub fn command_history_up(&mut self) {
        if self.command_history.is_empty() {
            return;
        }
        match self.history_index {
            None => {
                self.saved_input = self.command_input.clone();
                let idx = self.command_history.len() - 1;
                self.history_index = Some(idx);
                self.command_input = self.command_history[idx].clone();
                self.command_cursor = self.command_input.len();
            }
            Some(idx) if idx > 0 => {
                let new_idx = idx - 1;
                self.history_index = Some(new_idx);
                self.command_input = self.command_history[new_idx].clone();
                self.command_cursor = self.command_input.len();
            }
            _ => {}
        }
    }

    pub fn command_history_down(&mut self) {
        match self.history_index {
            Some(idx) => {
                if idx + 1 < self.command_history.len() {
                    let new_idx = idx + 1;
                    self.history_index = Some(new_idx);
                    self.command_input = self.command_history[new_idx].clone();
                    self.command_cursor = self.command_input.len();
                } else {
                    // Back to current input
                    self.history_index = None;
                    self.command_input = self.saved_input.clone();
                    self.command_cursor = self.command_input.len();
                }
            }
            None => {}
        }
    }

    /// Tab-complete the current input.
    pub fn tab_complete(&mut self) {
        let input = self.command_input.trim().to_lowercase();

        if self.tab_completions.is_empty() {
            // Build completion list
            self.tab_completions = COMMANDS
                .iter()
                .filter(|cmd| cmd.starts_with(&input))
                .map(|s| s.to_string())
                .collect();
            self.tab_completion_index = 0;
        } else {
            self.tab_completion_index =
                (self.tab_completion_index + 1) % self.tab_completions.len();
        }

        if let Some(completion) = self.tab_completions.get(self.tab_completion_index) {
            self.command_input = completion.clone();
            self.command_cursor = self.command_input.len();
        }
    }

    /// Execute the current command and produce output.
    pub fn execute_command(&mut self) {
        let cmd = self.command_input.trim().to_string();
        if cmd.is_empty() {
            return;
        }

        // Add to history (avoid duplicates of the last entry)
        if self.command_history.last().map(|s| s.as_str()) != Some(&cmd) {
            self.command_history.push(cmd.clone());
        }
        self.history_index = None;
        self.command_input.clear();
        self.command_cursor = 0;
        self.tab_completions.clear();

        // Parse and dispatch
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        match parts.first().map(|s| s.to_lowercase()).as_deref() {
            Some("help") => self.cmd_help(),
            Some("status") | Some("heartbeat") => self.cmd_status(),
            Some("view") => self.cmd_view(&parts),
            Some("logs") => self.cmd_logs(&parts),
            Some("clear") => self.cmd_clear(),
            Some("whoami") => self.cmd_whoami(),
            Some("theme") => self.cmd_theme(&parts),
            Some("snake") => self.cmd_snake(),
            Some("exit") | Some("quit") => {
                self.exit_command_mode();
                return;
            }
            _ => {
                self.set_output(vec![
                    OutputLine { text: format!("redwan@ssh:~$ {}", cmd), style: OutputStyle::Plain },
                    OutputLine { text: format!("Command not found: '{}'. Type 'help' for available commands.", cmd), style: OutputStyle::Error },
                ]);
            }
        }
    }

    fn set_output(&mut self, lines: Vec<OutputLine>) {
        self.output_chars_total = lines.iter().map(|l| l.text.len() + 1).sum::<usize>();
        self.output_lines = lines;
        self.output_chars_shown = 0;
        self.output_scroll = 0;
        self.mode = AppMode::TypewriterOutput;
    }

    fn cmd_help(&mut self) {
        self.set_output(vec![
            OutputLine { text: "╔══════════════════════════════════════════════════╗".into(), style: OutputStyle::Ascii },
            OutputLine { text: "║  REDWAN'S INDUSTRIAL TERMINAL — COMMAND MANUAL  ║".into(), style: OutputStyle::Ascii },
            OutputLine { text: "╚══════════════════════════════════════════════════╝".into(), style: OutputStyle::Ascii },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "  status / heartbeat    Live ESP32-Gateway telemetry".into(), style: OutputStyle::Data },
            OutputLine { text: "  view --pcb titan-core  ASCII PCB schematic viewer".into(), style: OutputStyle::Data },
            OutputLine { text: "  logs --mars            Mars Rover sensor data stream".into(), style: OutputStyle::Data },
            OutputLine { text: "  snake                  Play the Snake mini-game".into(), style: OutputStyle::Data },
            OutputLine { text: "  whoami                 Quick bio summary".into(), style: OutputStyle::Data },
            OutputLine { text: "  theme --matrix         Try the Matrix theme…".into(), style: OutputStyle::Data },
            OutputLine { text: "  clear                  Clear output pane".into(), style: OutputStyle::Data },
            OutputLine { text: "  exit                   Return to normal mode".into(), style: OutputStyle::Data },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "  [Tab] to auto-complete · [↑↓] for history".into(), style: OutputStyle::System },
        ]);
    }

    fn cmd_status(&mut self) {
        // Simulated ESP32-Gateway telemetry — uses animation_tick as a seed for variation
        let tick = self.animation_tick;
        let temp = 38 + (tick % 12) as u16;
        let uptime_h = 142 + (tick / 100) as u16;
        let uptime_m = (tick % 60) as u16;
        let signal = -40 - (tick % 30) as i16;
        let heap = 180_000 + (tick * 137 % 40_000);
        let pins: Vec<u8> = vec![4, 12, 15, 25, 33]
            .into_iter()
            .filter(|p| (tick + *p as usize) % 3 != 0)
            .collect();
        let pin_str: Vec<String> = pins.iter().map(|p| p.to_string()).collect();

        self.set_output(vec![
            OutputLine { text: "redwan@ssh:~$ status --remote".into(), style: OutputStyle::Plain },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "[SYSTEM]: Connecting to ESP32-Gateway v1.0...".into(), style: OutputStyle::System },
            OutputLine { text: "[SYSTEM]: Authenticating via MQTT broker...".into(), style: OutputStyle::System },
            OutputLine { text: "[OK] Connection Established.".into(), style: OutputStyle::Ok },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "┌──────────────────────────────────────────┐".into(), style: OutputStyle::Ascii },
            OutputLine { text: "│        ESP32-GATEWAY TELEMETRY           │".into(), style: OutputStyle::Header },
            OutputLine { text: "├──────────────────────────────────────────┤".into(), style: OutputStyle::Ascii },
            OutputLine { text: format!("│  [TEMP]:   {}°C                           │", temp), style: OutputStyle::Data },
            OutputLine { text: format!("│  [UPTIME]: {}h {}m                        │", uptime_h, uptime_m), style: OutputStyle::Data },
            OutputLine { text: format!("│  [WIFI]:   {} dBm (Connected)             │", signal), style: OutputStyle::Data },
            OutputLine { text: format!("│  [HEAP]:   {} bytes free                  │", heap), style: OutputStyle::Data },
            OutputLine { text: format!("│  [IO]:     Pins {} Pulsing               │", pin_str.join(", ")), style: OutputStyle::Data },
            OutputLine { text: "│  [MQTT]:   Subscribed (3 topics)          │".into(), style: OutputStyle::Data },
            OutputLine { text: "│  [STATUS]: ██████████ OPERATIONAL         │".into(), style: OutputStyle::Ok },
            OutputLine { text: "└──────────────────────────────────────────┘".into(), style: OutputStyle::Ascii },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "[SYSTEM]: Heartbeat OK. Next ping in 30s.".into(), style: OutputStyle::System },
        ]);
    }

    fn cmd_view(&mut self, parts: &[&str]) {
        let has_pcb = parts.iter().any(|p| *p == "--pcb");
        let has_titan = parts.iter().any(|p| p.to_lowercase() == "titan-core");

        if !has_pcb || !has_titan {
            self.set_output(vec![
                OutputLine { text: "Usage: view --pcb titan-core".into(), style: OutputStyle::Error },
                OutputLine { text: "Available boards: titan-core".into(), style: OutputStyle::Data },
            ]);
            return;
        }

        self.set_output(vec![
            OutputLine { text: "redwan@ssh:~$ view --pcb titan-core".into(), style: OutputStyle::Plain },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "[SYSTEM]: Loading PCB schematic...".into(), style: OutputStyle::System },
            OutputLine { text: "[OK] Titan-Core 4-Layer Motherboard Rev 2.1".into(), style: OutputStyle::Ok },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "  ┌─────────────────────────────────────────────────┐".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  ╔═══════════╗   ┌─────┐   ╔═══════════╗       │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  ║  ESP32-S3  ║───│ SPI │───║  W25Q128  ║  [ANT]│".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  ║  240MHz    ║   └─────┘   ║  16MB     ║   │   │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  ╚═════╤═════╝             ╚═══════════╝   │   │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │        │                                     │   │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  ┌─────┴─────┐  ┌──────────┐  ┌──────┐     │   │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  │   I2C     │  │  UART    │  │ ADC  │     │   │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  │  Bus      │  │  Bridge  │  │ 12b  │     │   │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  └─────┬─────┘  └────┬─────┘  └──┬───┘     │   │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │        │             │           │          │   │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  ╔═════╧═════════════╧═══════════╧══════╗   │   │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  ║          GPIO EXPANSION BUS          ║   │   │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  ║    4  12  15  25  26  27  32  33     ║───┘   │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │  ╚══════════════════════════════════════╝       │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │                                                 │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  │   [PWR]──┤3.3V LDO├──┤5V USB-C├──[GND]         │".into(), style: OutputStyle::Ascii },
            OutputLine { text: "  └─────────────────────────────────────────────────┘".into(), style: OutputStyle::Ascii },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "  Layer Stack: Signal / GND / PWR / Signal".into(), style: OutputStyle::Data },
            OutputLine { text: "  Designed in: KiCad 7.0  |  Manufactured: JLCPCB".into(), style: OutputStyle::Data },
        ]);
    }

    fn cmd_logs(&mut self, parts: &[&str]) {
        let has_mars = parts.iter().any(|p| *p == "--mars");
        if !has_mars {
            self.set_output(vec![
                OutputLine { text: "Usage: logs --mars".into(), style: OutputStyle::Error },
                OutputLine { text: "Available logs: --mars (Mars Rover / CanSat telemetry)".into(), style: OutputStyle::Data },
            ]);
            return;
        }

        let tick = self.animation_tick;
        // Generate mock sensor readings that look realistic
        let lat = 23.8103 + (tick % 100) as f64 * 0.0001;
        let lon = 90.4125 + (tick % 80) as f64 * 0.00012;
        let alt = 142.5 + (tick % 50) as f64 * 0.3;
        let pressure = 1013.25 - (tick % 40) as f64 * 0.15;
        let temp = 22.0 + (tick % 15) as f64 * 0.4;
        let accel_x = 0.02 + (tick % 7) as f64 * 0.003;
        let accel_z = 9.78 + (tick % 5) as f64 * 0.01;

        self.set_output(vec![
            OutputLine { text: "redwan@ssh:~$ logs --mars".into(), style: OutputStyle::Plain },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "[SYSTEM]: Accessing CanSat telemetry archive...".into(), style: OutputStyle::System },
            OutputLine { text: "[SYSTEM]: UIU Mars Rover — Electrical Team Logs".into(), style: OutputStyle::System },
            OutputLine { text: "[OK] Telemetry stream initialized.".into(), style: OutputStyle::Ok },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "┌──────────────────────────────────────────────┐".into(), style: OutputStyle::Ascii },
            OutputLine { text: "│    ◆ MISSION CONTROL — SENSOR READOUT ◆     │".into(), style: OutputStyle::Header },
            OutputLine { text: "├──────────────────────────────────────────────┤".into(), style: OutputStyle::Ascii },
            OutputLine { text: format!("│  T+00:00:01  LAT: {:.4}°N  LON: {:.4}°E   │", lat, lon), style: OutputStyle::Data },
            OutputLine { text: format!("│  T+00:00:02  ALT: {:.1}m  PRES: {:.2}hPa │", alt, pressure), style: OutputStyle::Data },
            OutputLine { text: format!("│  T+00:00:03  TEMP: {:.1}°C  HUM: 62.4%     │", temp), style: OutputStyle::Data },
            OutputLine { text: format!("│  T+00:00:04  ACCEL X: {:.3}g  Z: {:.2}g   │", accel_x, accel_z), style: OutputStyle::Data },
            OutputLine { text: "│  T+00:00:05  GYRO: 0.12°/s  MAG: 48.2µT    │".into(), style: OutputStyle::Data },
            OutputLine { text: "│  T+00:00:06  BATT: 11.8V  SOLAR: 4.2W      │".into(), style: OutputStyle::Data },
            OutputLine { text: "│  T+00:00:07  GPS FIX: 3D  SATS: 8          │".into(), style: OutputStyle::Data },
            OutputLine { text: "│  T+00:00:08  PARACHUTE: ARMED               │".into(), style: OutputStyle::Ok },
            OutputLine { text: "│  T+00:00:09  RADIO: 433MHz  RSSI: -67dBm   │".into(), style: OutputStyle::Data },
            OutputLine { text: "│  T+00:00:10  STATUS: NOMINAL                │".into(), style: OutputStyle::Ok },
            OutputLine { text: "└──────────────────────────────────────────────┘".into(), style: OutputStyle::Ascii },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "[SYSTEM]: End of archived stream. 847 packets logged.".into(), style: OutputStyle::System },
        ]);
    }

    fn cmd_clear(&mut self) {
        self.output_lines.clear();
        self.output_chars_shown = 0;
        self.output_chars_total = 0;
        self.output_scroll = 0;
        // Stay in command mode
        self.mode = AppMode::Command;
    }

    fn cmd_whoami(&mut self) {
        self.set_output(vec![
            OutputLine { text: "redwan@ssh:~$ whoami".into(), style: OutputStyle::Plain },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "╔══════════════════════════════════════════════╗".into(), style: OutputStyle::Ascii },
            OutputLine { text: "║  REDWAN AHMMED                               ║".into(), style: OutputStyle::Header },
            OutputLine { text: "╠══════════════════════════════════════════════╣".into(), style: OutputStyle::Ascii },
            OutputLine { text: "║  Role: R&D Hardware Specialist / Backend Dev ║".into(), style: OutputStyle::Data },
            OutputLine { text: "║  Org:  UIU Robotics                          ║".into(), style: OutputStyle::Data },
            OutputLine { text: "║  Edu:  CSE @ United International University ║".into(), style: OutputStyle::Data },
            OutputLine { text: "║  Spec: PCB Design · IoT · Embedded Systems   ║".into(), style: OutputStyle::Data },
            OutputLine { text: "║  Accolade: IAAC Bronze Honors                ║".into(), style: OutputStyle::Data },
            OutputLine { text: "╚══════════════════════════════════════════════╝".into(), style: OutputStyle::Ascii },
            OutputLine { text: String::new(), style: OutputStyle::Plain },
            OutputLine { text: "  \"Building the bridge between silicon and software.\"".into(), style: OutputStyle::System },
        ]);
    }

    fn cmd_theme(&mut self, parts: &[&str]) {
        let has_matrix = parts.iter().any(|p| *p == "--matrix");
        if has_matrix {
            self.set_output(vec![
                OutputLine { text: "redwan@ssh:~$ theme --matrix".into(), style: OutputStyle::Plain },
                OutputLine { text: String::new(), style: OutputStyle::Plain },
                OutputLine { text: "[SYSTEM]: Initializing Matrix theme...".into(), style: OutputStyle::System },
                OutputLine { text: "[SYSTEM]: Loading green phosphor palette...".into(), style: OutputStyle::System },
                OutputLine { text: String::new(), style: OutputStyle::Plain },
                OutputLine { text: "  ░▒▓█ MATRIX MODE ACTIVATED █▓▒░".into(), style: OutputStyle::Ok },
                OutputLine { text: String::new(), style: OutputStyle::Plain },
                OutputLine { text: "  ...just kidding.".into(), style: OutputStyle::Plain },
                OutputLine { text: String::new(), style: OutputStyle::Plain },
                OutputLine { text: "  Error: We prefer industrial-grade aesthetics here.".into(), style: OutputStyle::Error },
                OutputLine { text: "  This terminal runs on steel and solder, not neon.".into(), style: OutputStyle::System },
            ]);
        } else {
            self.set_output(vec![
                OutputLine { text: "Usage: theme --matrix".into(), style: OutputStyle::Error },
                OutputLine { text: "Available themes: --matrix (spoiler: it won't stay)".into(), style: OutputStyle::Data },
            ]);
        }
    }

    fn cmd_snake(&mut self) {
        self.mode = AppMode::SnakeGame;
        // Default game area — will be resized by the renderer
        self.snake_game = Some(SnakeGame::new(30, 15));
        self.snake_tick_counter = 0;
    }

    pub fn exit_snake(&mut self) {
        self.snake_game = None;
        self.mode = AppMode::Command;
    }

    // ── Typewriter output advancement ──────────────────────────

    /// Advance typewriter output. Returns true if still animating.
    pub fn advance_typewriter(&mut self, chars: usize) -> bool {
        if self.output_chars_shown < self.output_chars_total {
            self.output_chars_shown = (self.output_chars_shown + chars).min(self.output_chars_total);
            true
        } else {
            false
        }
    }

    /// Check if typewriter is finished.
    pub fn typewriter_done(&self) -> bool {
        self.output_chars_shown >= self.output_chars_total
    }

    // ── Project selection (Projects tab) ──────────────────────

    pub fn select_next_project(&mut self) {
        let total = content::total_project_count();
        if total > 0 && self.selected_project < total - 1 {
            self.selected_project += 1;
        }
    }

    pub fn select_prev_project(&mut self) {
        if self.selected_project > 0 {
            self.selected_project -= 1;
        }
    }

    pub fn select_first_project(&mut self) {
        self.selected_project = 0;
    }

    pub fn select_last_project(&mut self) {
        let total = content::total_project_count();
        if total > 0 {
            self.selected_project = total - 1;
        }
    }

    // ── Scrolling ──────────────────────────────────────────────

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self, content_height: usize, viewport_height: usize) {
        if content_height > viewport_height {
            let max = content_height - viewport_height;
            if self.scroll_offset < max {
                self.scroll_offset += 1;
            }
        }
    }

    pub fn content_line_count(&self) -> usize {
        match self.tab {
            Tab::Creations => content::total_project_lines(),
            Tab::Reflections => content::ABOUT_LINES.len(),
            Tab::Contact => {
                // intro + blank + entries + blank + outro
                1 + 1 + content::CONTACT_ENTRIES.len() + 1 + 1
            }
        }
    }

    // ── Intro animation ────────────────────────────────────────

    /// Advance the typewriter animation by `chars` characters.
    /// Returns `true` if the state changed (needs re-render).
    pub fn advance_intro(&mut self, chars: usize) -> bool {
        self.animation_tick = self.animation_tick.wrapping_add(1);
        match self.intro {
            IntroPhase::Typing { chars_shown } => {
                let total = content::banner_char_count();
                let next = (chars_shown + chars).min(total);
                if next >= total {
                    self.intro = IntroPhase::Pause { ticks_remaining: 8 };
                } else {
                    self.intro = IntroPhase::Typing { chars_shown: next };
                }
                true
            }
            IntroPhase::Pause { ticks_remaining } => {
                if ticks_remaining <= 1 {
                    self.intro = IntroPhase::Done;
                } else {
                    self.intro = IntroPhase::Pause {
                        ticks_remaining: ticks_remaining - 1,
                    };
                }
                true
            }
            IntroPhase::Done => true, // Still returning true because animation_tick changed
        }
    }

    /// Skip the intro animation immediately.
    pub fn skip_intro(&mut self) {
        self.intro = IntroPhase::Done;
    }

    pub fn intro_done(&self) -> bool {
        matches!(self.intro, IntroPhase::Done)
    }
}
