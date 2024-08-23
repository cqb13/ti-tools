pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightPurple,
    BrightCyan,
    BrightWhite,
    Reset,
}

impl Color {
    pub fn to_ansi_color_string(&self) -> String {
        match self {
            Color::Black => "\u{001b}[30m",
            Color::Red => "\u{001b}[31m",
            Color::Green => "\u{001b}[32m",
            Color::Yellow => "\u{001b}[33m",
            Color::Blue => "\u{001b}[34m",
            Color::Purple => "\u{001b}[35m",
            Color::Cyan => "\u{001b}[36m",
            Color::White => "\u{001b}[37m",
            Color::BrightBlack => "\u{001b}[90m",
            Color::BrightRed => "\u{001b}[91m",
            Color::BrightGreen => "\u{001b}[92m",
            Color::BrightYellow => "\u{001b}[93m",
            Color::BrightBlue => "\u{001b}[94m",
            Color::BrightPurple => "\u{001b}[95m",
            Color::BrightCyan => "\u{001b}[96m",
            Color::BrightWhite => "\u{001b}[97m",
            Color::Reset => "\u{001b}[0m",
        }
        .to_string()
    }

    pub fn from_string(string: String) -> Color {
        match string.as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "purple" => Color::Purple,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "bright-black" => Color::BrightBlack,
            "bright-red" => Color::BrightRed,
            "bright-green" => Color::BrightGreen,
            "bright-yellow" => Color::BrightYellow,
            "bright-blue" => Color::BrightBlue,
            "bright-purple" => Color::BrightPurple,
            "bright-cyan" => Color::BrightCyan,
            "bright-white" => Color::BrightWhite,
            "reset" => Color::Reset,
            _ => Color::Reset,
        }
    }

    pub fn to_ansi_background_color_string(&self) -> String {
        match self {
            Color::Black => "\u{001b}[40m",
            Color::Red => "\u{001b}[41m",
            Color::Green => "\u{001b}[42m",
            Color::Yellow => "\u{001b}[43m",
            Color::Blue => "\u{001b}[44m",
            Color::Purple => "\u{001b}[45m",
            Color::Cyan => "\u{001b}[46m",
            Color::White => "\u{001b}[47m",
            Color::BrightBlack => "\u{001b}[100m",
            Color::BrightRed => "\u{001b}[101m",
            Color::BrightGreen => "\u{001b}[102m",
            Color::BrightYellow => "\u{001b}[103m",
            Color::BrightBlue => "\u{001b}[104m",
            Color::BrightPurple => "\u{001b}[105m",
            Color::BrightCyan => "\u{001b}[106m",
            Color::BrightWhite => "\u{001b}[107m",
            Color::Reset => "\u{001b}[0m",
        }
        .to_string()
    }
}

pub enum TextStyle {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Invert,
    Hidden,
    Strikethrough,
    Reset,
}

impl TextStyle {
    pub fn to_ansi_style_string(&self) -> String {
        match self {
            TextStyle::Bold => "\u{001b}[1m",
            TextStyle::Dim => "\u{001b}[2m",
            TextStyle::Italic => "\u{001b}[3m",
            TextStyle::Underline => "\u{001b}[4m",
            TextStyle::Blink => "\u{001b}[5m",
            TextStyle::Invert => "\u{001b}[7m",
            TextStyle::Hidden => "\u{001b}[8m",
            TextStyle::Strikethrough => "\u{001b}[9m",
            TextStyle::Reset => "\u{001b}[0m",
        }
        .to_string()
    }

    pub fn from_string(string: String) -> TextStyle {
        match string.as_str() {
            "bold" => TextStyle::Bold,
            "dim" => TextStyle::Dim,
            "italic" => TextStyle::Italic,
            "underline" => TextStyle::Underline,
            "blink" => TextStyle::Blink,
            "invert" => TextStyle::Invert,
            "hidden" => TextStyle::Hidden,
            "strikethrough" => TextStyle::Strikethrough,
            "reset" => TextStyle::Reset,
            _ => TextStyle::Reset,
        }
    }
}

/*
ex:
[color:red]Hello[color:reset] [color:blue][bg:red]World
*/
#[macro_export]
macro_rules! prints {
    ($($arg:tt)*) => {{
        use $crate::styles::{Color, TextStyle};

        let mut result = String::new();
        let styled_string = format!($($arg)*);
        let mut i = 0;
        let len = styled_string.len();

        while i < len {
            if &styled_string[i..i + 1] == "[" {
                let end = styled_string[i..].find("]").unwrap() + i;
                let directive = &styled_string[i + 1..end];
                let parts: Vec<&str> = directive.split(':').collect();

                if parts.len() == 2 {
                    match parts[0] {
                        "color" => {
                            let color = Color::from_string(parts[1].to_string());
                            result.push_str(&color.to_ansi_color_string());
                        }
                        "bg" => {
                            let bg_color = Color::from_string(parts[1].to_string());
                            result.push_str(&bg_color.to_ansi_background_color_string());
                        }
                        "style" => {
                            let style = TextStyle::from_string(parts[1].to_string());
                            result.push_str(&style.to_ansi_style_string());
                        }
                        _ => {}
                    }
                }
                i = end + 1;
            } else {
                let end = styled_string[i..].find("[").unwrap_or(len - i) + i;
                result.push_str(&styled_string[i..end]);
                i = end;
            }
        }

        println!("{}{}", result, Color::Reset.to_ansi_color_string());
    }};
}
