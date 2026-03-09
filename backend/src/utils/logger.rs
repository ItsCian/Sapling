macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr, $text:expr) => {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", $r, $g, $b, $text)
    };
}
macro_rules! bold {
    ($text:expr) => {
        format!("\x1b[1m{}\x1b[0m", $text)
    };
}
macro_rules! dim {
    ($text:expr) => {
        format!("\x1b[2m{}\x1b[0m", $text)
    };
}

const SKY: (u8, u8, u8) = (147, 197, 253);
const PEACH: (u8, u8, u8) = (253, 210, 147);
const ROSE: (u8, u8, u8) = (255, 154, 162);
const LAVEND: (u8, u8, u8) = (197, 176, 253);
const MINT: (u8, u8, u8) = (154, 239, 195);
const MIST: (u8, u8, u8) = (200, 210, 220);
const SLATE: (u8, u8, u8) = (160, 170, 190);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Success,
}

impl Level {
    fn palette(self) -> (u8, u8, u8) {
        match self {
            Level::Info => SKY,
            Level::Warn => PEACH,
            Level::Error => ROSE,
            Level::Debug => LAVEND,
            Level::Success => MINT,
            Level::Trace => MIST,
        }
    }
    fn icon(self) -> &'static str {
        match self {
            Level::Info => "◆",
            Level::Warn => "▲",
            Level::Error => "✖",
            Level::Debug => "◎",
            Level::Success => "✔",
            Level::Trace => "·",
        }
    }
    fn tag(self) -> &'static str {
        match self {
            Level::Info => " INFO  ",
            Level::Warn => " WARN  ",
            Level::Error => " ERROR ",
            Level::Debug => " DEBUG ",
            Level::Success => "  OK   ",
            Level::Trace => " TRACE ",
        }
    }
}

pub struct Logger {
    pub min_level: Level,
    pub show_timestamp: bool,
    pub show_source: bool,
}

impl Logger {
    pub const fn new() -> Self {
        Logger {
            min_level: Level::Trace,
            show_timestamp: true,
            show_source: false,
        }
    }
    pub fn log(&self, level: Level, message: &str, file: &str, line: u32) {
        if level < self.min_level {
            return;
        }
        let (r, g, b) = level.palette();
        let ts = if self.show_timestamp {
            format!("{} ", dim!(rgb!(SLATE.0, SLATE.1, SLATE.2, now_hms())))
        } else {
            String::new()
        };
        let src = if self.show_source {
            format!(
                "  {}",
                dim!(rgb!(
                    SLATE.0,
                    SLATE.1,
                    SLATE.2,
                    format!("{}:{}", file, line)
                ))
            )
        } else {
            String::new()
        };
        let icon = rgb!(r, g, b, level.icon());
        let tag = bold!(rgb!(r, g, b, level.tag()));
        let msg = rgb!(r, g, b, message);
        println!("{ts}{icon} {tag}  {msg}{src}");
    }
}

static LOGGER: Logger = Logger::new();

pub fn init() {
    print_banner();
}
pub fn global() -> &'static Logger {
    &LOGGER
}

#[macro_export]
macro_rules! log_info    { ($($a:tt)*) => { $crate::utils::logger::global().log($crate::utils::logger::Level::Info,    &format!($($a)*), file!(), line!()) }; }
#[macro_export]
macro_rules! log_warn    { ($($a:tt)*) => { $crate::utils::logger::global().log($crate::utils::logger::Level::Warn,    &format!($($a)*), file!(), line!()) }; }
#[macro_export]
macro_rules! log_error   { ($($a:tt)*) => { $crate::utils::logger::global().log($crate::utils::logger::Level::Error,   &format!($($a)*), file!(), line!()) }; }
#[macro_export]
macro_rules! log_debug   { ($($a:tt)*) => { $crate::utils::logger::global().log($crate::utils::logger::Level::Debug,   &format!($($a)*), file!(), line!()) }; }
#[macro_export]
macro_rules! log_success { ($($a:tt)*) => { $crate::utils::logger::global().log($crate::utils::logger::Level::Success, &format!($($a)*), file!(), line!()) }; }
#[macro_export]
macro_rules! log_trace   { ($($a:tt)*) => { $crate::utils::logger::global().log($crate::utils::logger::Level::Trace,   &format!($($a)*), file!(), line!()) }; }

fn print_banner() {
    let rows: &[(&str, (u8, u8, u8))] = &[
        (
            "                                                                 ",
            SKY,
        ),
        (
            "  ███████╗ █████╗ ██████╗ ██╗     ██╗███╗   ██╗ ██████╗       ",
            (160, 190, 253),
        ),
        (
            "  ██╔════╝██╔══██╗██╔══██╗██║     ██║████╗  ██║██╔════╝       ",
            (175, 183, 253),
        ),
        (
            "  ███████╗███████║██████╔╝██║     ██║██╔██╗ ██║██║  ███╗      ",
            LAVEND,
        ),
        (
            "  ╚════██║██╔══██║██╔═══╝ ██║     ██║██║╚██╗██║██║   ██║      ",
            (185, 210, 220),
        ),
        (
            "  ███████║██║  ██║██║     ███████╗██║██║ ╚████║╚██████╔╝      ",
            (170, 230, 210),
        ),
        (
            "  ╚══════╝╚═╝  ╚═╝╚═╝     ╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝       ",
            MINT,
        ),
        (
            "                                                                 ",
            MINT,
        ),
    ];
    println!();
    for (row, (r, g, b)) in rows {
        println!("{}", rgb!(*r, *g, *b, bold!(row)));
    }
    let sub = "🌱  sapling backend  ·  axum + seaorm + postgresql  🌱";
    println!(
        "{}",
        rgb!(LAVEND.0, LAVEND.1, LAVEND.2, bold!(format!("{:^65}", sub)))
    );
    println!(
        "  {}",
        dim!(rgb!(SLATE.0, SLATE.1, SLATE.2, "─".repeat(63)))
    );
    println!();
}

fn now_hms() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let s = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{:02}:{:02}:{:02}", (s / 3600) % 24, (s / 60) % 60, s % 60)
}
