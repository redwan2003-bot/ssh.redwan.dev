use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{App, IntroPhase, Tab};
use crate::content;
use crate::theme;

fn portrait_width() -> u16 {
    content::PORTRAIT.iter().map(|l| l.len()).max().unwrap_or(0) as u16
}

fn portrait_height() -> u16 {
    content::PORTRAIT.len() as u16
}

/// Render the entire portfolio UI into the given frame.
pub fn render(app: &App, f: &mut Frame) {
    let area = f.area();

    // Clear the screen first
    f.render_widget(Clear, area);

    if !app.intro_done() {
        render_intro(app, f, area);
        return;
    }

    let p_width = portrait_width();
    let b_width = banner_width() as u16;
    
    // Switch to vertical if the terminal is too narrow for both portrait and a decent content area
    // A standard terminal is 80-120 chars. We try to be horizontal if >= 110
    let min_horizontal_width = 110;

    if area.width < min_horizontal_width || area.height < 30 {
        render_vertical(app, f, area);
    } else {
        render_horizontal(app, f, area, p_width, b_width);
    }
}

fn render_horizontal(app: &App, f: &mut Frame, area: Rect, p_width: u16, b_width: u16) {
    let main_layout = Layout::horizontal([
        Constraint::Length(p_width + 2), // Portrait + tighter padding
        Constraint::Min(40),             // Content column
    ])
    .split(area);

    let left_col = main_layout[0];
    let right_col = main_layout[1];

    // Render portrait in a rounded block
    render_portrait(f, left_col);

    let right_chunks = Layout::vertical([
        Constraint::Length(10),  // Logo (Reduced height for compactness)
        Constraint::Min(5),      // Content (Dynamic switch)
        Constraint::Length(3),   // Nav
        Constraint::Length(1),   // Footer
    ])
    .split(right_col);

    render_logo(app, f, right_chunks[0]);
    render_content(app, f, right_chunks[1]); // Switched from hardcoded render_bio
    render_tabs(app, f, right_chunks[2]);
    render_footer(app, f, right_chunks[3]);
}

fn render_vertical(app: &App, f: &mut Frame, area: Rect) {
    let p_height = portrait_height();
    let l_height = banner_height();

    let chunks = Layout::vertical([
        Constraint::Max(p_height + 2),    // Portrait (Cap height in vertical mode)
        Constraint::Length(l_height + 2), // Logo (Tighter padding)
        Constraint::Min(10),              // Content
        Constraint::Length(3),            // Nav
        Constraint::Length(1),            // Footer
    ])
    .split(area);

    render_portrait(f, chunks[0]);
    render_logo(app, f, chunks[1]);
    render_content(app, f, chunks[2]); // Switched from hardcoded render_bio
    render_tabs(app, f, chunks[3]);
    render_footer(app, f, chunks[4]);
}

fn render_portrait(f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .padding(Padding::new(0, 0, 0, 0)); // Zero internal padding for compactness
    
    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines: Vec<Line> = content::PORTRAIT
        .iter()
        .map(|&l| Line::from(Span::styled(l, theme::TEXT))) // Brighter style for better visibility
        .collect();
    // Use no wrapping for portrait to keep ASCII art intact, 
    // it will be truncated if area is too small
    let text = Paragraph::new(Text::from(lines))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    f.render_widget(text, inner);
}

fn render_logo(app: &App, f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .padding(Padding::new(1, 1, 0, 0));
        
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Render floating stars background
    render_stars(app, f, inner);

    let lines: Vec<Line> = content::BANNER
        .iter()
        .map(|&l| Line::from(Span::styled(l, theme::ACCENT)))
        .collect();
    let text = Paragraph::new(Text::from(lines)).alignment(Alignment::Left);
    f.render_widget(text, inner);
}

fn render_stars(app: &App, f: &mut Frame, area: Rect) {
    if area.width < 10 || area.height < 3 {
        return;
    }

    let tick = app.animation_tick;
    let symbols = ["*", "+", ".", "·", "°"];
    let star_count = 8;

    for i in 0..star_count {
        // Deterministic but "random-looking" positions
        let seed = i * 137;
        let x_speed = (i % 3) + 1;
        
        // Floating effect: horizontal drift + subtle vertical wobble
        let x = ((seed + tick / x_speed) % (area.width as usize)) as u16;
        let y_wobble = if (tick / 10 + i) % 2 == 0 { 1 } else { 0 };
        let y = ((seed / 7 + y_wobble) % (area.height as usize)) as u16;

        let symbol = symbols[i % symbols.len()];
        let style = if i % 2 == 0 { theme::TEXT_DIM } else { theme::TEXT_MUTED };

        let star_area = Rect {
            x: area.x + x,
            y: area.y + y,
            width: 1,
            height: 1,
        };

        if star_area.x < area.x + area.width && star_area.y < area.y + area.height {
            f.render_widget(Paragraph::new(Span::styled(symbol, style)), star_area);
        }
    }
}

fn render_bio(f: &mut Frame, area: Rect) {
    let lines: Vec<Line> = content::ABOUT_LINES
        .iter()
        .map(|&l| Line::from(Span::styled(l, theme::TEXT)))
        .collect();
    let text = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false });
    f.render_widget(text, area);
}

// ── Intro animation ────────────────────────────────────────────

fn render_intro(app: &App, f: &mut Frame, area: Rect) {
    let total = 500; // arbitrary total for intro
    let chars_shown = match app.intro {
        IntroPhase::Typing { chars_shown } => chars_shown,
        IntroPhase::Pause { .. } | IntroPhase::Done => total,
    };

    let max_w = banner_width();

    // Build the revealed portion of the banner.
    // Pad each line to the same width so centering keeps alignment.
    let mut lines: Vec<Line> = Vec::new();
    let mut remaining = chars_shown;

    for &banner_line in content::BANNER {
        if remaining == 0 {
            break;
        }
        let show = remaining.min(banner_line.len());
        let revealed = &banner_line[..show];

        // Pad the revealed portion to max banner width
        let padded = format!("{:<width$}", revealed, width = max_w);
        let mut spans = vec![Span::styled(padded, theme::HEADER)];

        // Show a blinking cursor at the end of the current typing line
        if show < banner_line.len() {
            spans.push(Span::styled("\u{2588}", theme::INTRO_CURSOR));
        }

        lines.push(Line::from(spans));

        // consume chars + 1 for the implicit newline
        remaining = remaining.saturating_sub(banner_line.len() + 1);
    }

    let banner_h = lines.len() as u16 + 2; // +2 for borders
    let banner_w = (max_w as u16 + 4).min(area.width); // +4 for padding/borders

    let x_offset = area.width.saturating_sub(banner_w) / 2;
    let y_offset = area.height.saturating_sub(banner_h) / 2;

    let banner_area = Rect {
        x: area.x + x_offset,
        y: area.y + y_offset,
        width: banner_w,
        height: banner_h.min(area.height.saturating_sub(y_offset)),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .padding(Padding::new(2, 2, 0, 0));

    let inner = block.inner(banner_area);
    f.render_widget(block, banner_area);

    let text = Paragraph::new(Text::from(lines)).alignment(Alignment::Center);
    f.render_widget(text, inner);
}

// ── Header (ASCII banner) ──────────────────────────────────────

fn banner_height() -> u16 {
    content::BANNER.len() as u16
}

fn banner_width() -> usize {
    content::BANNER.iter().map(|l| l.len()).max().unwrap_or(0)
}

fn render_header(f: &mut Frame, area: Rect) {
    let max_w = banner_width();

    // Pad each banner line to the same width so Alignment::Center
    // shifts them as a uniform block instead of centering each
    // line independently (which breaks the ASCII art).
    let lines: Vec<Line> = content::BANNER
        .iter()
        .map(|&l| {
            let padded = format!("{:<width$}", l, width = max_w);
            Line::from(Span::styled(padded, theme::HEADER))
        })
        .collect();

    // Pad subtitle to the same width for consistent centering
    let subtitle_raw = "software engineer  \u{00b7}  France";
    let sub_pad_total = max_w.saturating_sub(subtitle_raw.chars().count());
    let sub_pad_left = sub_pad_total / 2;
    let sub_pad_right = sub_pad_total - sub_pad_left;

    let subtitle = Line::from(vec![
        Span::raw(" ".repeat(sub_pad_left)),
        Span::styled("software engineer", theme::TEXT_DIM),
        Span::styled("  \u{00b7}  ", theme::TEXT_MUTED),
        Span::styled("France", theme::TEXT_DIM),
        Span::raw(" ".repeat(sub_pad_right)),
    ]);

    let mut all_lines = lines;
    all_lines.push(Line::from(format!("{:<width$}", "", width = max_w)));
    all_lines.push(subtitle);

    let text = Paragraph::new(Text::from(all_lines))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(theme::BORDER),
        );
    f.render_widget(text, area);
}

// ── Tab bar ────────────────────────────────────────────────────

fn render_tabs(app: &App, f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER);
    
    let inner = block.inner(area);
    f.render_widget(block, area);

    let tabs: Vec<Span> = Tab::ALL
        .iter()
        .enumerate()
        .flat_map(|(i, t)| {
            let num = Span::styled(format!(" {} ", i + 1), theme::TAB_NUMBER);
            let style = if *t == app.tab {
                theme::TAB_ACTIVE
            } else {
                theme::TAB_INACTIVE
            };
            let label = Span::styled(format!(" {} ", t.label()), style);
            let spacer = Span::raw("  ");
            vec![num, label, spacer]
        })
        .collect();

    let line = Line::from(tabs);
    let paragraph = Paragraph::new(line).alignment(Alignment::Center);
    f.render_widget(paragraph, inner);
}

// ── Content area ───────────────────────────────────────────────

fn render_content(app: &App, f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    match app.tab {
        Tab::Creations => render_projects(app, f, inner),
        Tab::Reflections => render_about(app, f, inner),
        Tab::Contact => render_contact(app, f, inner),
    }
}

// ── About tab ──────────────────────────────────────────────────

fn render_about(app: &App, f: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    for (i, &line_str) in content::ABOUT_LINES.iter().enumerate() {
        if i == 0 {
            // First line is the greeting — make it bold
            lines.push(Line::from(Span::styled(line_str, theme::TEXT_BOLD)));
        } else if line_str.is_empty() {
            lines.push(Line::from(""));
        } else {
            lines.push(Line::from(Span::styled(line_str, theme::TEXT)));
        }
    }

    let total_lines = lines.len();
    let viewport_h = area.height as usize;

    let text = Paragraph::new(Text::from(lines))
        .scroll((app.scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });
    f.render_widget(text, area);

    if total_lines > viewport_h {
        render_scroll_indicator(f, area, app.scroll_offset, total_lines, viewport_h);
    }
}

// ── Projects tab (telescope-style split pane) ──────────────────

fn render_projects(app: &App, f: &mut Frame, area: Rect) {
    // Split into left (40%) and right (60%) panes
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    render_project_list(app, f, panes[0]);
    render_project_detail(app, f, panes[1]);
}

/// Render the left pane: project list grouped by category.
fn render_project_list(app: &App, f: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(theme::BORDER)
        .padding(Padding::new(1, 1, 0, 0));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();
    let mut flat_idx: usize = 0;

    for (cat_idx, cat) in content::PROJECT_CATEGORIES.iter().enumerate() {
        if cat_idx > 0 {
            lines.push(Line::from(""));
        }

        // Category header (non-selectable)
        lines.push(Line::from(Span::styled(cat.name, theme::CATEGORY_HEADER)));

        for project in cat.projects.iter() {
            let is_selected = flat_idx == app.selected_project;

            let line = if is_selected {
                Line::from(vec![
                    Span::styled(" \u{25b8} ", theme::PROJECT_ARROW),
                    Span::styled(project.name, theme::PROJECT_SELECTED),
                ])
            } else {
                Line::from(vec![
                    Span::raw("   "),
                    Span::styled(project.name, theme::PROJECT_LIST_ITEM),
                ])
            };

            lines.push(line);
            flat_idx += 1;
        }
    }

    let total_lines = lines.len();
    let viewport_h = inner.height as usize;

    // Auto-scroll to keep the selected project visible.
    // Find which line the selected project is on.
    let selected_line = find_selected_line_in_list(app.selected_project);
    let scroll_offset = compute_auto_scroll(selected_line, viewport_h, total_lines);

    let text = Paragraph::new(Text::from(lines)).scroll((scroll_offset as u16, 0));
    f.render_widget(text, inner);
}

/// Given a flat project index, compute which line it falls on in the list pane.
fn find_selected_line_in_list(selected: usize) -> usize {
    let mut line: usize = 0;
    let mut flat_idx: usize = 0;

    for (cat_idx, cat) in content::PROJECT_CATEGORIES.iter().enumerate() {
        if cat_idx > 0 {
            line += 1; // blank separator
        }
        line += 1; // category header

        for _ in cat.projects.iter() {
            if flat_idx == selected {
                return line;
            }
            line += 1;
            flat_idx += 1;
        }
    }
    line
}

/// Compute a scroll offset that keeps `target_line` visible within the viewport,
/// trying to center it when possible.
fn compute_auto_scroll(target_line: usize, viewport_h: usize, total_lines: usize) -> usize {
    if total_lines <= viewport_h {
        return 0;
    }
    let max_scroll = total_lines.saturating_sub(viewport_h);
    // Try to center the target line
    let ideal = target_line.saturating_sub(viewport_h / 2);
    ideal.min(max_scroll)
}

/// Render the right pane: detail view for the selected project.
fn render_project_detail(app: &App, f: &mut Frame, area: Rect) {
    let block = Block::default().padding(Padding::new(2, 1, 1, 0));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let Some((category, project)) = content::get_project_by_flat_index(app.selected_project) else {
        return;
    };

    let mut lines: Vec<Line> = Vec::new();

    // Project name
    lines.push(Line::from(Span::styled(
        project.name,
        theme::PROJECT_DETAIL_NAME,
    )));

    // Category
    lines.push(Line::from(Span::styled(
        category.name,
        theme::PROJECT_DETAIL_CATEGORY,
    )));

    lines.push(Line::from(""));

    // Description
    lines.push(Line::from(Span::styled(project.description, theme::TEXT)));

    lines.push(Line::from(""));

    // Tech stack
    lines.push(Line::from(vec![
        Span::styled("Tech  ", theme::PROJECT_DETAIL_LABEL),
        Span::styled(project.tech, theme::TEXT_DIM),
    ]));

    lines.push(Line::from(""));

    // URL
    lines.push(Line::from(vec![
        Span::styled("URL   ", theme::PROJECT_DETAIL_LABEL),
        Span::styled(project.url, theme::LINK),
    ]));

    let text = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false });
    f.render_widget(text, inner);
}

// ── Skills tab ─────────────────────────────────────────────────

fn render_skills(app: &App, f: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    for (i, group) in content::SKILLS.iter().enumerate() {
        if i > 0 {
            lines.push(Line::from(""));
        }

        lines.push(Line::from(Span::styled(group.name, theme::SKILL_GROUP)));

        let items_str = group.items.join("  \u{00b7}  ");
        lines.push(Line::from(Span::styled(items_str, theme::SKILL_ITEM)));
    }

    let total_lines = lines.len();
    let viewport_h = area.height as usize;

    let text = Paragraph::new(Text::from(lines))
        .scroll((app.scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });
    f.render_widget(text, area);

    if total_lines > viewport_h {
        render_scroll_indicator(f, area, app.scroll_offset, total_lines, viewport_h);
    }
}

// ── Contact tab ────────────────────────────────────────────────

fn render_contact(app: &App, f: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::styled(
        "Want to get in touch? Here's where you can find me:",
        theme::TEXT,
    )));
    lines.push(Line::from(""));

    for entry in content::CONTACT_ENTRIES {
        lines.push(Line::from(vec![
            Span::styled(format!("  {:<10}", entry.label), theme::CONTACT_LABEL),
            Span::styled(entry.value, theme::LINK),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        content::CONTACT_OUTRO,
        theme::TEXT_DIM,
    )));

    let total_lines = lines.len();
    let viewport_h = area.height as usize;

    let text = Paragraph::new(Text::from(lines))
        .scroll((app.scroll_offset as u16, 0))
        .wrap(Wrap { trim: false });
    f.render_widget(text, area);

    if total_lines > viewport_h {
        render_scroll_indicator(f, area, app.scroll_offset, total_lines, viewport_h);
    }
}

// ── Footer ─────────────────────────────────────────────────────

fn render_footer(_app: &App, f: &mut Frame, area: Rect) {
    let spans = vec![
        Span::styled("[", theme::TEXT_MUTED),
        Span::styled("\u{2190} \u{2192} ", theme::KEY_HINT),
        Span::styled("to select · ", theme::TEXT),
        Span::styled("enter ", theme::KEY_HINT),
        Span::styled("to open · ", theme::TEXT),
        Span::styled("q ", theme::KEY_HINT),
        Span::styled("to quit", theme::TEXT),
        Span::styled("]", theme::TEXT_MUTED),
    ];

    let help = Paragraph::new(Line::from(spans)).alignment(Alignment::Left);
    f.render_widget(help, area);
}

// ── Scroll indicator ───────────────────────────────────────────

fn render_scroll_indicator(
    f: &mut Frame,
    area: Rect,
    offset: usize,
    total: usize,
    viewport: usize,
) {
    let max_scroll = total.saturating_sub(viewport);
    if max_scroll == 0 {
        return;
    }

    // Show a small position indicator at the top-right of the content area
    let pct = if max_scroll > 0 {
        (offset * 100) / max_scroll
    } else {
        0
    };

    let indicator = if offset == 0 {
        "\u{2191} top".to_string()
    } else if offset >= max_scroll {
        "\u{2193} end".to_string()
    } else {
        format!("{}%", pct)
    };

    let indicator_area = Rect {
        x: area.x + area.width.saturating_sub(indicator.len() as u16 + 1),
        y: area.y,
        width: (indicator.len() as u16).min(area.width),
        height: 1,
    };

    let text = Paragraph::new(Span::styled(indicator, theme::SCROLL_INDICATOR));
    f.render_widget(text, indicator_area);
}
