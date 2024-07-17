#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    Clear, // Reset to default color
}

impl Color {
    pub fn to_ansi_color_code(&self) -> &str {
        match self {
            Color::Black => "\u{001b}[30m",
            Color::Red => "\u{001b}[31m",
            Color::Green => "\u{001b}[32m",
            Color::Yellow => "\u{001b}[33m",
            Color::Blue => "\u{001b}[34m",
            Color::Purple => "\u{001b}[35m",
            Color::Cyan => "\u{001b}[36m",
            Color::White => "\u{001b}[37m",
            Color::Clear => "\u{001b}[0m",
        }
    }

    pub fn to_ansi_background_color_code(&self) -> &str {
        match self {
            Color::Black => "\u{001b}[40m",
            Color::Red => "\u{001b}[41m",
            Color::Green => "\u{001b}[42m",
            Color::Yellow => "\u{001b}[43m",
            Color::Blue => "\u{001b}[44m",
            Color::Purple => "\u{001b}[45m",
            Color::Cyan => "\u{001b}[46m",
            Color::White => "\u{001b}[47m",
            Color::Clear => "\u{001b}[0m",
        }
    }
}
