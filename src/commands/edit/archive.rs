use super::{load_program, save_edits};
use std::path::Path;

pub fn archive_command(input_path_string: String, new_file_path: Option<String>, delete_old: bool) {
    let input_path = Path::new(&input_path_string).to_path_buf();
    let mut program = load_program(&input_path);

    program.metadata.archive();

    save_edits(program, &input_path, new_file_path, delete_old);

    println!("Set the program destination to archive.");
}
