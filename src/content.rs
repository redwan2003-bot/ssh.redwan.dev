// ── ASCII Art Banner ───────────────────────────────────────────
//
// Compact "slant"-style banner for the header.
// Each line is a &str so the renderer can reveal them character by
// character during the intro animation.

pub const BANNER: &[&str] = &[
    r"  _____          _                                _                                  _ ",
    r" |  __ \        | |                         /\   | |                                | |",
    r" | |__) |___  __| |_      ____ _ _ __      /  \  | |__  _ __ ___  _ __ ___   ___  __| |",
    r" |  _  // _ \/ _` \ \ /\ / / _` | '_ \    / /\ \ | '_ \| '_ ` _ \| '_ ` _ \ / _ \/ _` |",
    r" | | \ \  __/ (_| |\ V  V / (_| | | | |  / ____ \| | | | | | | | | | | | | |  __/ (_| |",
    r" |_|  \_\___|\__,_| \_/\_/ \__,_|_| |_| /_/    \_\_| |_|_| |_| |_|_| |_| |_|\___|\__,_|",

];

/// Total number of characters in the banner (for the typewriter animation).
pub fn banner_char_count() -> usize {
    BANNER.iter().map(|l| l.len()).sum::<usize>() + BANNER.len() // +newlines
}

pub const PORTRAIT: &[&str] = &[
    r"                        .......                             ",
    r"                    ................                        ",
    r"                  ...          .....:.                      ",
    r"                 :.    ..:::--:::.. ..                      ",
    r"                :.. .::---========:...:                     ",
    r"                ...:=-:--====+=====:..:                     ",
    r"                :..:+-::--===-::::+=::-.                    ",
    r"                 ..-=....::--::-==++-:.                     ",
    r"                  ..::..:::----::-=+-::.                    ",
    r"                  .-+----::-=+-::-=++===.                   ",
    r"                 ::-+=--::::-:-====+++==.                   ",
    r"                 .::--:::..:----::==++=.                    ",
    r"                  .::-::..-++===--==+.                      ",
    r"                     .:::::...-===++:                       ",
    r"                     .=-----======-=:                       ",
    r"                      .-:.......::-=:                       ",
    r"                      :::-:...:---==*-                      ",
    r"                   :++=.:::::-----=+*#***-.                 ",
    r"              :++++====:.:::::---=***********+.             ",
    r"         ..*++++++++=-==::::::::=*****************:         ",
    r"      .=*+++++=++++++=------:::-+**************+*****+:     ",
    r"     +++++++++=++++++==--===--=+++*************+**+*****.   ",
    r"    *+++++++++++++++++++==-=++++++++++********++++++*++**.  ",
    r"   -*=-++++++=+++++++++++==++++++++++++*******++++++++++**  ",
    r"   ++=-=+=+++++++++++++++++++*++++++++++*****++++++++++++*- ",
    r"  +++=--=-+++++++++++++++==+++++++++++++++***++++++++++++*+ ",
    r"  =++=--=:=====+++++++++++=++++++++++++++++**++++=+++++++*+ ",
    r" ==+++=--:-====++++++++++++++++=+++++++++++++==+-===++++++++",
    r" ==++=---:-=====++++++++++++++==++++++++++++++==-====++++++*",
];

pub const ABOUT_LINES: &[&str] = &[
    "is a Robotics-Hardware Specialist (R&D) & Backend Developer,",
    "building high-performance robotics,",
    "documenting industrial IoT & reflecting on how",
    "embedded systems bridge the physical divide.",
    "",
    "He currently serves as an R&D Hardware Specialist at",
    "UIU Robotics, where he architected the Titan-Core",
    "4-layer motherboard & the ESP32-Gateway v1.0.",
    "",
    "Previously, Redwan earned Bronze Honors in the",
    "International Astronomy & Astrophysics Competition (IAAC)",
    "and studying CSE at United International University",
    "where he was research intern in the Electrical team for the Mars Rover and",
    "CanSat teams.",
    "",
    "His work sits at the intersection of",
    "industrial-grade PCB design, IoT infrastructure,",
    "and competitive robotics. Explore the",
    "directories below to learn more ↓",
];

// ── Projects ───────────────────────────────────────────────────

pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub tech: &'static str,
    pub url: &'static str,
}

pub struct ProjectCategory {
    pub name: &'static str,
    pub projects: &'static [Project],
}

pub const PROJECT_CATEGORIES: &[ProjectCategory] = &[
    ProjectCategory {
        name: "IoT",
        projects: &[
            Project {
                name: "Air Mouse B.V.",
                description: "Python and esp32 based air mouse",
                tech: "Python, Flask, ESP32",
                url: "https://github.com/redwan2003-bot/Air_Mouse_B.V.",
            },
            Project {
                name: "Voice-Assistant-Stellar-AI-Lander-Style",
                description: "Ai voice assistant in stellar lander style as mini desk companion",
                tech: "C++, ESP-IDF",
                url: "https://github.com/redwan2003-bot/DIY-Voice-Assistant-Stellar-AI-Lander-Style",
            },
            Project {
                name: "ESP32-GateWay",
                description: "IoT Gateway for industrial automation",
                tech: "C++, ESP-IDF",
                url: "https://github.com/redwan2003-bot/Esp32-Gateway-IoT",
            },
        ],
    },
    //ProjectCategory {
        //name: "SoftWare",
        //projects: &[
            //Project {
            //    name: "mira",
            //    description: "Cross-platform streaming app for movies and TV shows",
            //    tech: "React Native, TypeScript",
            //    url: "https://github.com/YannickHerrero/mira",
            //},
            //Project {
            //    name: "miru",
            //    description: "Terminal-native anime streaming CLI with Anilist + Real-Debrid",
            //    tech: "Rust",
            //    url: "https://github.com/YannickHerrero/miru",
            //},
       //],
    //},
    // ProjectCategory {
    //     name: "Terminal Fun",
    //     projects: &[
    //         Project {
    //             name: "Solaris",
    //             description: "Terminal idle game — harness the cosmos to generate energy",
    //             tech: "Rust, ratatui",
    //             url: "https://github.com/YannickHerrero/Solaris",
    //         },
    //         Project {
    //             name: "Balatrust",
    //             description: "A terminal-based Balatro clone",
    //             tech: "Rust",
    //             url: "https://github.com/YannickHerrero/Balatrust",
    //         },
    //         Project {
    //             name: "kanitomo",
    //             description: "Terminal mini-game collection with your pet crab companion",
    //             tech: "Rust",
    //             url: "https://github.com/YannickHerrero/kanitomo",
    //         },
    //     ],
    // },
    // ProjectCategory {
    //     name: "Tools & Productivity",
    //     projects: &[
    //         Project {
    //             name: "mtools",
    //             description: "Unified toolkit for work management and developer utilities",
    //             tech: "TypeScript",
    //             url: "https://github.com/YannickHerrero/mtools",
    //         },
    //         Project {
    //             name: "motionflow",
    //             description: "Pipeline for generating short-form French educational videos",
    //             tech: "TypeScript, AI",
    //             url: "https://github.com/YannickHerrero/motionflow",
    //         },
    //         Project {
    //             name: "life",
    //             description: "Personal habit tracking for learning, nutrition, and sport",
    //             tech: "TypeScript",
    //             url: "https://github.com/YannickHerrero/life",
    //         },
    //     ],
    // },
    // ProjectCategory {
    //     name: "Web & Config",
    //     projects: &[
    //         Project {
    //             name: "yannickh.dev",
    //             description: "Personal portfolio and project showcase",
    //             tech: "Next.js, TypeScript, Vercel",
    //             url: "https://yannickh.dev",
    //         },
    //         Project {
    //             name: "ssh-yannickh.dev",
    //             description: "This SSH portfolio you're looking at right now",
    //             tech: "Rust, russh, ratatui, Fly.io",
    //             url: "https://github.com/YannickHerrero/ssh-yannickh.dev",
    //         },
    //         Project {
    //             name: "windot",
    //             description: "Windows/WSL dotfiles with tiling WM and custom status bar",
    //             tech: "JavaScript, PowerShell",
    //             url: "https://github.com/YannickHerrero/windot",
    //         },
    //         Project {
    //             name: "chocofi-config",
    //             description: "ZMK firmware config for Corne/Chocofi split keyboard",
    //             tech: "ZMK, Devicetree",
    //             url: "https://github.com/YannickHerrero/chocofi-config",
    //         },
    //     ],
    // },
];

/// Flat count of all projects across categories.
pub fn total_project_lines() -> usize {
    let mut lines = 0;
    for (i, cat) in PROJECT_CATEGORIES.iter().enumerate() {
        if i > 0 {
            lines += 1; // blank separator between categories
        }
        lines += 1; // category header
        lines += 1; // blank after header
        for (j, _) in cat.projects.iter().enumerate() {
            if j > 0 {
                lines += 1; // blank between projects
            }
            lines += 4; // name, desc, tech, url
        }
    }
    lines
}

/// Total number of projects across all categories.
pub fn total_project_count() -> usize {
    PROJECT_CATEGORIES
        .iter()
        .map(|cat| cat.projects.len())
        .sum()
}

/// Map a flat project index (0-based) to the corresponding category and project.
/// Returns `None` if the index is out of range.
pub fn get_project_by_flat_index(
    idx: usize,
) -> Option<(&'static ProjectCategory, &'static Project)> {
    let mut remaining = idx;
    for cat in PROJECT_CATEGORIES {
        if remaining < cat.projects.len() {
            return Some((cat, &cat.projects[remaining]));
        }
        remaining -= cat.projects.len();
    }
    None
}

// ── Skills ─────────────────────────────────────────────────────

pub struct SkillGroup {
    pub name: &'static str,
    pub items: &'static [&'static str],
}

pub const SKILLS: &[SkillGroup] = &[
    SkillGroup {
        name: "Languages",
        items: &["Rust", "TypeScript", "JavaScript", "Lua", "CSS"],
    },
    SkillGroup {
        name: "Mobile",
        items: &["React Native", "Expo"],
    },
    SkillGroup {
        name: "Web",
        items: &["Next.js", "Astro", "React"],
    },
    SkillGroup {
        name: "Terminal / TUI",
        items: &["ratatui", "crossterm", "russh"],
    },
    SkillGroup {
        name: "Infrastructure",
        items: &["Docker", "Vercel", "GitHub Actions", "Linux", "Arch"],
    },
    SkillGroup {
        name: "IoT",
        items: &["Arduino", "ESP32", "ESP8266", "Pcb Design", "Raspberry Pi","STM32","PIC18F"],
    },
    //SkillGroup {
    //    name: "Other",
    //    items: &["ZMK firmware", "Base16 theming", "WSL / Hyprland"],
    //},
];

// ── Contact ────────────────────────────────────────────────────

pub struct ContactEntry {
    pub label: &'static str,
    pub value: &'static str,
}

pub const CONTACT_ENTRIES: &[ContactEntry] = &[
    ContactEntry {
        label: "Medium",
        value: "https://medium.com/@reahs302444",
    },
    ContactEntry {
        label: "GitHub",
        value: "https://github.com/redwan2003-bot",
    },
    ContactEntry {
        label: "Gmail",
        value: "reahs302444@gmail.com",
    },
    ContactEntry {
        label: "LinkedIn",
        value: "https://www.linkedin.com/in/redwanahmmed/",
    },
];

pub const CONTACT_OUTRO: &str = "Open to freelance opportunities — feel free to reach out.";
