use super::{DisplayMode, EncodeMode};
use crate::errors::CliError;
use crate::tokens::Map;

struct EncodeState<'a> {
    in_string: bool,
    in_evaluated_string: bool,
    in_custom_name: bool,
    in_list_name: bool,
    last_token_key: Vec<u8>,
    current_mode: &'a EncodeMode,
    default_mode: &'a EncodeMode,
}

impl<'a> EncodeState<'a> {
    fn new(mode: &'a EncodeMode) -> Self {
        Self {
            in_string: false,
            in_evaluated_string: false,
            in_custom_name: false,
            in_list_name: false,
            last_token_key: Vec::new(),
            current_mode: &mode,
            default_mode: &mode,
        }
    }

    fn default_is_smart(&self) -> bool {
        match self.default_mode {
            EncodeMode::Smart => true,
            _ => false,
        }
    }

    fn reset(&mut self) {
        self.in_string = false;
        self.in_evaluated_string = false;
        self.in_custom_name = false;
        self.in_list_name = false;
        self.last_token_key = Vec::new();
        self.current_mode = self.default_mode;
    }
}

pub fn encode(
    decoded_program: &String,
    tokens: &Map,
    perform_normalize: bool,
    display_mode: DisplayMode,
    encode_mode: &EncodeMode,
) -> Result<Vec<u8>, CliError> {
    let mut encoded_program = Vec::new();

    let decoded_program = if perform_normalize {
        normalize(&decoded_program)
    } else {
        decoded_program.to_string()
    };

    let mut state = EncodeState::new(&encode_mode);
    for line in decoded_program.lines() {
        let mut temp_line = line.to_string();

        while !temp_line.is_empty() {
            let token: Option<(String, String)> = match state.current_mode {
                EncodeMode::Max => tokens.get_longest_matching_token(&temp_line, &display_mode),
                EncodeMode::Min => tokens.get_shortest_matching_token(&temp_line, &display_mode),
                EncodeMode::Smart => tokens.get_longest_matching_token(&temp_line, &display_mode),
            };

            match token {
                Some((key, value)) => {
                    let key_as_bytes = convert_key_to_bytes(&key);

                    if state.default_is_smart() {
                        if state.last_token_key == vec![0x5F] || state.last_token_key == vec![0xEB]
                        {
                            // prgm and ʟ
                            state.in_custom_name = true;
                            state.current_mode = &EncodeMode::Max;
                        } else if key_as_bytes == vec![0x2A] {
                            state.in_string = !state.in_string;
                            // Send(
                            state.in_evaluated_string =
                                state.in_string && state.last_token_key == vec![0xE7];
                            state.current_mode = &EncodeMode::Min;
                        } else if key_as_bytes == vec![0x04] {
                            // -> / →
                            state.reset();
                        } else if !value.chars().all(|c| c.is_ascii_alphabetic()) {
                            state.reset()
                        }
                    }

                    encoded_program.extend(&key_as_bytes);
                    state.last_token_key = key_as_bytes;
                    temp_line = temp_line[value.len()..].to_string();
                }
                None => {
                    return Err(CliError::TokenNotFound(temp_line));
                }
            }
        }
        state.reset();
        encoded_program.push(0x3F); // New line
    }

    encoded_program.pop(); // Remove last new line

    Ok(encoded_program)
}

fn normalize(string: &str) -> String {
    let string = string
        .replace('\u{0398}', "θ")
        .replace('\u{03F4}', "θ")
        .replace('\u{1DBF}', "θ");
    string
}

fn convert_key_to_bytes(key: &str) -> Vec<u8> {
    let key = key.replace(" en", "");
    let keys = key.split("$").collect::<Vec<&str>>();
    let keys = keys[1..].to_vec();
    let mut bytes = Vec::new();

    for key in keys {
        let byte = u8::from_str_radix(key, 16).unwrap();
        bytes.push(byte);
    }

    bytes
}
