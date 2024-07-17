use super::{load_program, save_edits};
use crate::commands::exit_with_error;
use std::path::Path;

pub fn unarchive_command(
    input_path_string: String,
    new_file_path: Option<String>,
    delete_old: bool,
) {
    let input_path = Path::new(&input_path_string).to_path_buf();
    let mut program = load_program(&input_path);

    let result = program.metadata.unarchive();

    match result {
        Ok(_) => {}
        Err(err) => exit_with_error(&err),
    }

    save_edits(program, &input_path, new_file_path, delete_old);

    println!("Set program destination to RAM.");
}
