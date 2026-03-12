use crate::content;

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
}

impl App {
    pub fn new() -> Self {
        Self {
            tab: Tab::Reflections,
            should_quit: false,
            scroll_offset: 0,
            intro: IntroPhase::Typing { chars_shown: 0 },
            selected_project: 0,
            animation_tick: 0,
        }
    }

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
