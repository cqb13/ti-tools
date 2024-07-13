use super::exit_with_error;
use crate::calculator::models::Model;
use crate::calculator::program::Program;
use crate::calculator::DisplayMode;
use crate::tokens::OsVersion;
use std::path::Path;

pub fn details_command(input_path_string: String) {
    let target_version = OsVersion {
        model: Model::Latest,
        version: "latest".to_string(),
    };

    let input_path = Path::new(&input_path_string);

    let program = Program::load_from_8xp(
        input_path.to_path_buf(),
        target_version,
        DisplayMode::Accessible,
    );

    let program = match program {
        Ok(program) => program,
        Err(err) => exit_with_error(&err),
    };

    println!("{}", program.metadata.name);
    println!("{}", program.header.comment);
    println!(
        "Total Size: {} bytes",
        program.header.bytes.len()
            + program.metadata.bytes.len()
            + program.body.bytes.len()
            + program.checksum.bytes.len()
    );
    println!("Body Size: {} bytes", program.body.bytes.len());
    println!("----- Status -----");
    println!("{}", program.metadata.archived.to_string());
    println!("{}", program.metadata.file_type.to_string())
}
