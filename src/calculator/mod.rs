mod create;
mod decode;
mod encode;
pub mod models;
pub mod program;

pub enum EncodeMode {
    Min,
    Max,
    Smart,
}

impl EncodeMode {
    pub fn from_string(encode_mode: &str) -> Result<EncodeMode, String> {
        match encode_mode {
            "min" => Ok(EncodeMode::Min),
            "max" => Ok(EncodeMode::Max),
            "smart" => Ok(EncodeMode::Smart),
            _ => Err(format!("Unknown encode mode: {}", encode_mode)),
        }
    }
}

pub enum DisplayMode {
    Pretty,
    Accessible,
    TiAscii,
}

impl DisplayMode {
    pub fn from_string(display_mode: &str) -> Result<DisplayMode, String> {
        match display_mode {
            "pretty" => Ok(DisplayMode::Pretty),
            "accessible" => Ok(DisplayMode::Accessible),
            "ti" => Ok(DisplayMode::TiAscii),
            _ => Err(format!("Unknown display mode: {}", display_mode)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DisplayMode::Pretty => "pretty",
            DisplayMode::Accessible => "accessible",
            DisplayMode::TiAscii => "ti",
        }
        .to_string()
    }
}
