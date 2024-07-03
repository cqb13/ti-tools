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

    //TODO use better encoding method
    for line in decoded_program.lines() {
        let mut temp_line = line.to_string();
        while !temp_line.is_empty() {
            let token = tokens.get_longest_matching_token(&temp_line, &display_mode);

            match token {
                Some((key, value)) => {
                    encoded_program.extend(convert_key_to_bytes(&key));
                    temp_line = temp_line[value.len()..].to_string();
                }
                None => {
                    println!("Error: Could not find token for line: {}", temp_line);
                    std::process::exit(1);
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
