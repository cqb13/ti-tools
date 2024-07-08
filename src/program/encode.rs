use super::DisplayMode;
use crate::tokens::Map;

pub enum EncodeMode {
    Min,
    Max,
    Smart,
}

pub fn encode(
    decoded_program: &String,
    tokens: &Map,
    perform_normalize: bool,
    display_mode: DisplayMode,
    encode_mode: EncodeMode,
) -> Vec<u8> {
    let mut encoded_program = Vec::new();

    let decoded_program = if perform_normalize {
        normalize(&decoded_program)
    } else {
        decoded_program.to_string()
    };

    for line in decoded_program.lines() {
        let mut in_string = false;
        let mut temp_line = line.to_string();

        while !temp_line.is_empty() {
            let token = match encode_mode {
                EncodeMode::Max => tokens.get_longest_matching_token(&temp_line, &display_mode),
                EncodeMode::Min => tokens.get_shortest_matching_token(&temp_line, &display_mode),
                EncodeMode::Smart => {
                    if in_string {
                        tokens.get_shortest_matching_token(&temp_line, &display_mode)
                    } else {
                        tokens.get_longest_matching_token(&temp_line, &display_mode)
                    }
                }
            };

            match token {
                Some((key, value)) => {
                    if value == "\"" {
                        in_string = !in_string
                    }

                    encoded_program.extend(convert_key_to_bytes(&key));
                    temp_line = temp_line[value.len()..].to_string();
                }
                None => {
                    println!("Error: Could not find token for line: {}", temp_line);
                    std::process::exit(1);
                }
            }
        }

        encoded_program.push(0x3F); // New line
    }

    encoded_program.pop(); // Remove last new line

    encoded_program
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
