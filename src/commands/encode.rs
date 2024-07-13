use super::exit_with_error;
use crate::calculator::program::Program;
use crate::calculator::EncodeMode;
use std::path::Path;

pub fn encode_command(
    input_path_string: String,
    output_path_string: Option<String>,
    encode_mode: String,
    content: bool,
    preview: bool,
) {
    let encode_mode = match EncodeMode::from_string(&encode_mode) {
        Ok(encode_mode) => encode_mode,
        Err(err) => exit_with_error(&err),
    };

    let input_path = Path::new(&input_path_string);

    let program = Program::load_from_txt(input_path.to_path_buf(), encode_mode);

    let program = match program {
        Ok(program) => program,
        Err(err) => exit_with_error(&err),
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
                Err(err) => exit_with_error(&err),
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
