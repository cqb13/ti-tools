use super::exit_with_error;
use crate::calculator::program::Program;
use crate::calculator::DisplayMode;
use std::path::Path;

pub fn details_command(input_path_string: String) {
    let input_path = Path::new(&input_path_string);

    let program = Program::load_from_8xp(input_path.to_path_buf(), DisplayMode::Accessible);

    let program = match program {
        Ok(program) => program,
        Err(err) => exit_with_error(&err),
    };

    println!("{}", program.metadata.name);
    println!("{}", program.header.comment);
    println!(
        "Total Size: {} bytes",
        program.header.bytes.len() + program.metadata.bytes.len() + program.body.bytes.len() + 2
    ); // 2 for checksum
    println!("Body Size: {} bytes", program.body.bytes.len());
    println!("----- Status -----");
    println!("model: {}", program.model.model.to_string());
    println!("language: {}", program.model.language);
    println!("Destination: {}", program.metadata.destination.to_string());
    println!("File Type: {}", program.metadata.file_type.to_string())
}
