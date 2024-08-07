use crate::errors::CliError;
use serde::{Deserialize, Serialize};

mod create;
mod decode;
mod encode;
pub mod models;
pub mod program;

#[derive(Serialize)]
pub enum EncodeMode {
    Min,
    Max,
    Smart,
}

impl EncodeMode {
    pub fn from_string(encode_mode: &str) -> Result<EncodeMode, CliError> {
        match encode_mode {
            "min" => Ok(EncodeMode::Min),
            "max" => Ok(EncodeMode::Max),
            "smart" => Ok(EncodeMode::Smart),
            _ => Err(CliError::Match(
                encode_mode.to_string(),
                "EncodeMode".to_string(),
            )),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DisplayMode {
    Pretty,
    Accessible,
    TiAscii,
}

impl DisplayMode {
    pub fn from_string(display_mode: &str) -> Result<DisplayMode, CliError> {
        match display_mode {
            "pretty" => Ok(DisplayMode::Pretty),
            "accessible" => Ok(DisplayMode::Accessible),
            "ti" => Ok(DisplayMode::TiAscii),
            _ => Err(CliError::Match(
                display_mode.to_string(),
                "DisplayMode".to_string(),
            )),
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
