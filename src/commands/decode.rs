use super::exit_with_error;
use crate::calculator::program::Program;
use crate::calculator::{DisplayMode, Model};
use crate::tokens::OsVersion;
use std::path::Path;

pub fn decode_command(
    input_path_string: String,
    output_path_string: Option<String>,
    display_mode_string: String,
    model: String,
    content: bool,
    preview: bool,
) {
    let model = match Model::from_string(&model) {
        Ok(model) => model,
        Err(err) => exit_with_error(&err),
    };

    let target_version = OsVersion {
        model,
        version: "latest".to_string(),
    };

    let input_path = Path::new(&input_path_string);

    let display_mode = match DisplayMode::from_string(&display_mode_string) {
        Ok(display_mode) => display_mode,
        Err(err) => exit_with_error(&err),
    };

    let program = Program::load_from_8xp(input_path.to_path_buf(), target_version, display_mode);

    let program = match program {
        Ok(program) => program,
        Err(err) => exit_with_error(&err),
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

            if display_mode_string == "pretty" {
                println!("Warning: Pretty tokens can't be accurately encoded")
            }

            match result {
                Ok(_) => {}
                Err(err) => exit_with_error(&err),
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
