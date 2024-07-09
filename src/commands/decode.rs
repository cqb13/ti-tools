use crate::program::{DisplayMode, Program};
use crate::tokens::OsVersion;
use std::path::Path;

pub fn decode_command(
    input_path_string: String,
    output_path_string: Option<String>,
    display_mode: String,
    model: String,
    content: bool,
    preview: bool,
) {
    let target_version = OsVersion {
        model,
        version: "latest".to_string(),
    };

    let input_path = Path::new(&input_path_string);

    let display_mode = DisplayMode::from_string(&display_mode);

    let program = Program::load_from_8xp(input_path.to_path_buf(), target_version, display_mode);

    let program = match program {
        Ok(program) => program,
        Err(err) => {
            println!("{}", err);
            std::process::exit(0);
        }
    };

    if content {
        let mut bytes: Vec<&u8> = Vec::new();
        bytes.extend(&program.header.bytes);
        bytes.extend(&program.metadata.bytes);
        bytes.extend(&program.body.bytes);
        bytes.extend(&program.checksum.bytes);

        print_bytes(bytes);
        println!();
    }

    if preview {
        println!("{}", program.display());
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

    println!("Successfully converted to txt")
}

fn print_bytes(bytes: Vec<&u8>) {
    let mut i = 0;
    for byte in bytes {
        print!("{:02X}", byte);
        i += 1;
        if i % 16 == 0 {
            println!();
        } else {
            print!(", ");
        }
    }
}
