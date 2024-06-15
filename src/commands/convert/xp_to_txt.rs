use crate::commands::convert::print_bytes;
use crate::tokens::single_byte_tokens::SingleByteTokens;
use std::path::PathBuf;

pub fn convert_8xp_to_txt(input_path: PathBuf, bytes: bool) {
    let file = match std::fs::read(&input_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to read file: {}", err);
            std::process::exit(0);
        }
    };

    if bytes {
        print_bytes(&file);
    }

    let single_byte_tokens = SingleByteTokens::new();

    println!("Tokens:");
    println!();
    for byte in file {
        let token = single_byte_tokens.get_token(byte);

        match token {
            Ok(token) => {
                if token == "\n" {
                    println!();
                } else {
                    print!("{}", token);
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                std::process::exit(0);
            }
        }
    }

    println!();
}
