use crate::commands::convert::print_bytes;
use crate::tokens::Tokens;
use std::path::PathBuf;

pub fn convert_8xp_to_txt(
    input_path: PathBuf,
    bytes: bool,
    display: bool,
    log_messages: bool,
) -> Vec<String> {
    let file = match std::fs::read(&input_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to read file: {}", err);
            std::process::exit(0);
        }
    };

    if bytes {
        println!("Bytes:");
        print_bytes(&file);
        println!();
    }

    let tokens = Tokens::new();

    if display {
        println!("Tokens:");
        println!();
    }
    let mut output_file = Vec::new();
    let mut skip_next = false;
    for (i, byte) in file.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        let token = tokens.single_byte_tokens.get(byte);

        match token {
            Some(token) => match token.as_str() {
                "[error: unused code point]" => {
                    if log_messages {
                        println!("Unused code point");
                    }
                }
                "[error: unknown 2-byte code]" => {
                    if log_messages {
                        println!(
                            "Unknown 2-byte code point: {:02X?}|{:02X?}",
                            byte,
                            file[i + 1]
                        );
                    }
                    skip_next = true;
                }
                _ => {
                    if display {
                        if token.to_string() == "\n" {
                            println!();
                        } else {
                            print!("{}", token.to_string());
                        }
                    }

                    output_file.push(token.to_string());
                }
            },
            None => {
                if log_messages {
                    println!("Unknown code point: {:02X?}", byte);
                }
            }
        }
    }
    if display {
        println!();
    }

    output_file
}
