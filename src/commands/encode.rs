use crate::program::{DisplayMode, Program};
use crate::tokens::OsVersion;
use std::path::Path;

pub fn encode_command(
    input_path_string: String,
    output_path_string: Option<String>,
    model: String,
    content: bool,
    preview: bool,
) {
    let target_version = OsVersion {
        model,
        version: "latest".to_string(),
    };

    let input_path = Path::new(&input_path_string);

    let program = Program::load(
        input_path.to_path_buf(),
        target_version,
        DisplayMode::Accessible,
    );

    let program = match program {
        Ok(program) => program,
        Err(err) => {
            println!("{}", err);
            std::process::exit(0);
        }
    };

    if content {
        println!("{}", program.display());
        println!();
    }

    if preview {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(&program.header.bytes);
        bytes.extend(&program.metadata.bytes);
        bytes.extend(&program.body.bytes);
        bytes.extend(&program.checksum.bytes);

        print_bytes(&bytes);
        println!();
    }

    match output_path_string {
        Some(output_path_string) => {
            let output_path = Path::new(&output_path_string);
            let result = program.save_to(output_path.to_path_buf());

            match result {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(0);
                }
            }
        }
        None => {}
    }

    println!("Successfully converted to 8xp")
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
