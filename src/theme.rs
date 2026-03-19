use ratatui::style::{Color, Modifier, Style};

// ── Colors ─────────────────────────────────────────────────────
pub const BG: Color = Color::Reset;
pub const FG: Color = Color::White;
pub const ACCENT: Color = Color::Cyan;
pub const DIM: Color = Color::Indexed(244); // Medium gray
pub const MUTED: Color = Color::Indexed(238); // Dark gray
pub const BORDER_COLOR: Color = Color::Rgb(60, 60, 60);

// ── Header ─────────────────────────────────────────────────────
pub const HEADER: Style = Style::new().fg(FG).add_modifier(Modifier::BOLD);

// ── Tabs ───────────────────────────────────────────────────────
pub const TAB_ACTIVE: Style = Style::new()
    .fg(ACCENT)
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::REVERSED);
pub const TAB_INACTIVE: Style = Style::new().fg(DIM);
pub const TAB_NUMBER: Style = Style::new().fg(MUTED);

// ── Borders ────────────────────────────────────────────────────
pub const BORDER: Style = Style::new().fg(BORDER_COLOR);

// ── Text ───────────────────────────────────────────────────────
pub const TEXT: Style = Style::new().fg(FG);
pub const TEXT_DIM: Style = Style::new().fg(DIM);
pub const TEXT_MUTED: Style = Style::new().fg(MUTED);
pub const TEXT_BOLD: Style = Style::new().fg(FG).add_modifier(Modifier::BOLD);

// ── Links ──────────────────────────────────────────────────────
pub const LINK: Style = Style::new()
    .fg(Color::Rgb(100, 149, 237)) // Cornflower Blue
    .add_modifier(Modifier::UNDERLINED);

// ── Projects ───────────────────────────────────────────────────
pub const CATEGORY_HEADER: Style = Style::new().fg(ACCENT).add_modifier(Modifier::BOLD);
pub const PROJECT_SELECTED: Style = Style::new().fg(ACCENT).add_modifier(Modifier::BOLD);
pub const PROJECT_LIST_ITEM: Style = Style::new().fg(FG);
pub const PROJECT_ARROW: Style = Style::new().fg(ACCENT).add_modifier(Modifier::BOLD);
pub const PROJECT_DETAIL_NAME: Style = Style::new().fg(FG).add_modifier(Modifier::BOLD);
pub const PROJECT_DETAIL_LABEL: Style = Style::new()
    .fg(DIM)
    .add_modifier(Modifier::BOLD);
pub const PROJECT_DETAIL_CATEGORY: Style = Style::new().fg(ACCENT);

// ── Skills ─────────────────────────────────────────────────────
pub const SKILL_GROUP: Style = Style::new().fg(ACCENT).add_modifier(Modifier::BOLD);
pub const SKILL_ITEM: Style = Style::new().fg(FG);

// ── Contact ────────────────────────────────────────────────────
pub const CONTACT_LABEL: Style = Style::new()
    .fg(DIM)
    .add_modifier(Modifier::BOLD);

// ── Footer ─────────────────────────────────────────────────────
pub const KEY_HINT: Style = Style::new().fg(FG).add_modifier(Modifier::BOLD);
pub const KEY_ACTION: Style = Style::new().fg(DIM);

// ── Scroll indicator ───────────────────────────────────────────
pub const SCROLL_INDICATOR: Style = Style::new().fg(MUTED);

// ── Intro animation ────────────────────────────────────────────
pub const INTRO_CURSOR: Style = Style::new()
    .fg(FG)
    .add_modifier(Modifier::RAPID_BLINK);

// ── Command prompt ─────────────────────────────────────────────
pub const CMD_PROMPT: Style = Style::new()
    .fg(Color::Rgb(0, 255, 136)) // Neon green prompt
    .add_modifier(Modifier::BOLD);
pub const CMD_INPUT: Style = Style::new().fg(FG);
pub const CMD_CURSOR: Style = Style::new()
    .fg(Color::Rgb(0, 255, 136))
    .add_modifier(Modifier::RAPID_BLINK);
pub const CMD_HINT: Style = Style::new().fg(MUTED);

// ── Typewriter output styles ───────────────────────────────────
pub const OUTPUT_SYSTEM: Style = Style::new().fg(ACCENT);
pub const OUTPUT_OK: Style = Style::new()
    .fg(Color::Rgb(0, 255, 136)) // Bright green
    .add_modifier(Modifier::BOLD);
pub const OUTPUT_DATA: Style = Style::new().fg(Color::Rgb(255, 200, 50)); // Amber/gold
pub const OUTPUT_ERROR: Style = Style::new()
    .fg(Color::Rgb(255, 80, 80)) // Soft red
    .add_modifier(Modifier::BOLD);
pub const OUTPUT_ASCII: Style = Style::new().fg(Color::Rgb(100, 120, 140)); // Steel blue-gray
pub const OUTPUT_HEADER: Style = Style::new()
    .fg(ACCENT)
    .add_modifier(Modifier::BOLD);

// ── Snake game ─────────────────────────────────────────────────
pub const SNAKE_HEAD: Style = Style::new()
    .fg(Color::Rgb(0, 255, 136))
    .add_modifier(Modifier::BOLD);
pub const SNAKE_BODY: Style = Style::new().fg(Color::Rgb(0, 200, 100));
pub const SNAKE_FOOD: Style = Style::new()
    .fg(Color::Rgb(255, 80, 80))
    .add_modifier(Modifier::BOLD);
pub const SNAKE_BORDER: Style = Style::new().fg(Color::Rgb(80, 80, 80));
pub const SNAKE_SCORE: Style = Style::new()
    .fg(Color::Rgb(255, 200, 50))
    .add_modifier(Modifier::BOLD);
pub const SNAKE_GAME_OVER: Style = Style::new()
    .fg(Color::Rgb(255, 80, 80))
    .add_modifier(Modifier::BOLD);
