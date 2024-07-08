use super::DisplayMode;
use crate::tokens::Map;

pub fn encode(
    decoded_program: &String,
    tokens: &Map,
    perform_normalize: bool,
    display_mode: DisplayMode,
) -> Vec<u8> {
    let mut encoded_program = Vec::new();

    let decoded_program = if perform_normalize {
        normalize(&decoded_program)
    } else {
        decoded_program.to_string()
    };

    for line in decoded_program.lines() {
        let mut string_indices = Vec::new();

        for (index, character) in line.chars().enumerate() {
            if character == '"' {
                string_indices.push(index);
            }
        }

        let mut line_segments = Vec::new();
        let mut in_string = false;
        let mut string = Vec::new();
        for (index, character) in line.chars().enumerate() {
            if string_indices.contains(&index) {
                line_segments.push((string.join(""), in_string));
                in_string = !in_string;
                string.clear();
            }

            string.push(character.to_string());

            if index + 1 == line.len() {
                line_segments.push((string.join(""), false))
            }
        }

        for (segment_string, in_string) in line_segments {
            let mut temp_segment_string = segment_string.to_string();

            while !temp_segment_string.is_empty() {
                let token = if !in_string {
                    tokens.get_longest_matching_token(&temp_segment_string, &display_mode)
                } else {
                    tokens.get_shortest_matching_token(&temp_segment_string, &display_mode)
                };

                match token {
                    Some((key, value)) => {
                        encoded_program.extend(convert_key_to_bytes(&key));
                        temp_segment_string = temp_segment_string[value.len()..].to_string();
                    }
                    None => {
                        println!(
                            "Error: Could not find token for line: {}",
                            temp_segment_string
                        );
                        std::process::exit(1);
                    }
                }
            }
        }
        encoded_program.push(0x3F);
    }

    encoded_program.pop();

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