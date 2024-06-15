use crate::commands::convert::print_bytes;
use crate::tokens::single_byte_tokens::SingleByteTokens;
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
    for byte in file {
        let token = single_byte_tokens.get_token(byte);

        match token {
            Ok(token) => {
                if display {
                    if token == "\n" {
                        println!();
                    } else {
                        print!("{}", token);
                    }
                }

                output_file.push(token);
            }
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
