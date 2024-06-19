use crate::commands::convert::print_bytes;
use crate::tokens::single_byte_tokens::{SingleByteToken, SingleByteTokens};
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

    let single_byte_tokens = SingleByteTokens::new();

    if display {
        println!("Tokens:");
        println!();
    }
    let mut output_file = Vec::new();
    for (i, byte) in file.iter().enumerate() {
        let token = single_byte_tokens.get_token(*byte);

        match token {
            Ok(token) => match token {
                SingleByteToken::UnusedCodePoint => {
                    if log_messages {
                        println!("Unused code point");
                    }
                }
                SingleByteToken::Unknown2ByteCode => {
                    if log_messages {
                        println!(
                            "Unknown 2-byte code point: {:02X?}|{:02X?}",
                            byte,
                            file[i + 1]
                        );
                    }
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
            Err(err) => {
                println!("Error: {}", err);
                std::process::exit(0);
            }
        }
    }
    if display {
        println!();
    }

    output_file
}
