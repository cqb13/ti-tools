use crate::tokens::TokensReversed;
use std::path::PathBuf;

pub fn convert_txt_to_8xp(input_path: PathBuf, raw: bool, display: bool) -> Vec<String> {
    let file = match std::fs::read_to_string(&input_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to read file: {}", err);
            std::process::exit(0);
        }
    };

    if raw {
        println!("File content:");
        for line in file.split("\n") {
            println!("{}", line)
        }
        println!("\n");
    }

    let body_bytes = translate_body(file);
    print_bytes(&body_bytes);

    Vec::new()
}

fn print_bytes(file: &Vec<u8>) {
    let mut i = 0;
    for byte in file {
        print!("{:02X}", byte);
        i += 1;
        if i % 16 == 0 {
            println!();
        } else {
            print!(", ");
        }
    }
}

fn translate_body(file: String) -> Vec<u8> {
    let mut body_bytes: Vec<u8> = Vec::new();
    let tokens = TokensReversed::new();

    let mut start_translating = false;
    for line in file.lines() {
        if start_translating {
            let mut temp_line = line.to_string();
            while !temp_line.is_empty() {
                let mut longest_token = "";
                let mut longest_token_value = Vec::new();
                let mut longest_token_length = 0;

                let token_sources = [
                    (&tokens.single_byte_tokens, None),
                    (&tokens.misc_tokens, Some(0xBB)),
                    (&tokens.equation_tokens, Some(0x5E)),
                    (&tokens.statistic_variable_tokens, Some(0x62)),
                    (&tokens.ti_84_tokens, Some(0xEF)),
                    (&tokens.window_and_finance_tokens, Some(0x63)),
                    (&tokens.string_tokens, Some(0xAA)),
                    (&tokens.picture_tokens, Some(0x60)),
                    (&tokens.matrix_tokens, Some(0x5C)),
                    (&tokens.list_tokens, Some(0x5D)),
                    (&tokens.graph_format_tokens, Some(0x7E)),
                    (&tokens.gdb_tokens, Some(0x61)),
                ];

                for (token_map, prefix) in &token_sources {
                    for token in token_map.keys() {
                        if temp_line.starts_with(token) && token.len() > longest_token_length {
                            longest_token_value = match prefix {
                                Some(p) => vec![*p, *token_map.get(token).unwrap()],
                                None => vec![*token_map.get(token).unwrap()],
                            };
                            longest_token = token;
                            longest_token_length = token.len();
                        }
                    }
                }

                if longest_token.is_empty() {
                    eprintln!("Error: Could not find token for line: {}", temp_line);
                    std::process::exit(1);
                }

                // Add the token's corresponding byte to the body bytes
                body_bytes.extend(longest_token_value.iter().copied());

                // Remove the found token from the line
                temp_line = temp_line[longest_token_length..].to_string();
            }

            body_bytes.push(0x3F);
        }

        if line.contains("Program Start:") {
            start_translating = true;
        }
    }

    body_bytes.pop(); // Remove the last 0x3F

    body_bytes
}

fn create_comment() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("Created by TI-Tools {}", version)
}
