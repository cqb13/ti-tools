use super::print_bytes;
use crate::tokens::TokensReversed;
use std::path::PathBuf;

pub fn convert_txt_to_8xp(input_path: PathBuf, name: String, raw: bool, display: bool) -> Vec<u8> {
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

    let body = translate_body(file);
    let checksum = (body.len() as u16).to_le_bytes().to_vec();
    let metadata = create_metadata(name, body.len() as u16, 2);
    let header = create_header(metadata.len() as u16, body.len() as u16);

    if header.len() != 55 {
        println!("Something went wrong while generating header");
        std::process::exit(0);
    }

    if metadata.len() != 19 {
        println!("Something went wrong while generating metadata");
        std::process::exit(0);
    }

    let mut output = Vec::new();
    output.extend(header);
    output.extend(metadata);
    output.extend(body);
    output.extend(checksum);

    if display {
        print_bytes(&output);
    }

    output
}

// 55 bytes
fn create_header(metadata_length: u16, body_length: u16) -> Vec<u8> {
    let mut header: Vec<u8> = Vec::new();
    let signature = "**TI83F*";
    header.extend(signature.as_bytes());
    header.extend([0x1A, 0x0A]); // signature part 2
    header.push(0x00); // mystery byte
    let mut comment = create_comment().as_bytes().to_owned();
    while comment.len() != 42 {
        comment.push(0x00)
    }
    header.extend(comment);

    let metadata_and_body_length = (metadata_length + body_length).to_le_bytes().to_vec();
    header.extend(metadata_and_body_length);
    header
}

fn create_comment() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("Created by TI-Tools {}", version)
}

// 19 byte header
fn create_metadata(name: String, body_length: u16, checksum_length: u16) -> Vec<u8> {
    let mut metadata: Vec<u8> = Vec::new();
    metadata.push(0x0D); // flag
    metadata.push(0x00); // unknown byte
    let body_and_checksum_length = (body_length + checksum_length).to_le_bytes().to_vec();
    metadata.extend(&body_and_checksum_length);
    metadata.push(0x05); // file type (0x05: normal, 0x06: edit-locked, 0x17: groups)

    let mut name_as_bytes = name.to_ascii_uppercase().as_bytes().to_owned();

    while name_as_bytes.len() != 8 {
        name_as_bytes.push(0x00)
    }

    metadata.extend(name_as_bytes);
    metadata.push(0x00); // version
    metadata.push(0x00); // archived (0x00: normal, 0x80: archived)
    metadata.extend(body_and_checksum_length); // again for some reason
    let body_length_as_two_byte = body_length.to_le_bytes().to_vec();
    metadata.extend(body_length_as_two_byte);
    metadata
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
