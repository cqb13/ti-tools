use super::DisplayMode;
use crate::errors::CliError;
use crate::tokens::Map;

pub fn decode(
    bytestream: &[u8],
    map: &Map,
    lang: &str,
    mode: &DisplayMode,
) -> Result<String, CliError> {
    let mut decoded_program = String::new();
    let mut index = 0;
    let mut current_bytes = Vec::new();

    while index < bytestream.len() {
        current_bytes.push(bytestream[index]);

        let key = match current_bytes.len() {
            1 => format!("${:02X}", current_bytes[0]),
            2 => format!("${:02X}${:02X}", current_bytes[0], current_bytes[1]),
            _ => {
                return Err(CliError::InvalidByteLength(format!(
                    "{:02X?}",
                    current_bytes
                )))
            }
        };

        let token = map.get_value(format!("{} {}", key, lang).as_str());

        match token {
            Some(token) => {
                let representation = match mode {
                    DisplayMode::Pretty => &token.display,
                    DisplayMode::Accessible => &token.accessible,
                    DisplayMode::TiAscii => &token.ti_ascii,
                };
                decoded_program.push_str(representation);
                current_bytes.clear();
            }
            None => {}
        }

        index += 1;
    }

    if current_bytes.is_empty() {
        Ok(decoded_program)
    } else {
        Err(CliError::TokenNotFound(format!("{:02X?}", current_bytes)))
    }
}
