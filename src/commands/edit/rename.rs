use super::{load_program, save_edits};
use std::path::Path;

pub fn rename_command(
    input_path_string: String,
    name: String,
    new_file_path: Option<String>,
    delete_old: bool,
) {
    let input_path = Path::new(&input_path_string).to_path_buf();
    let mut program = load_program(&input_path);

    let result = program.metadata.rename(name);

    match result {
        Ok(_) => {}
        Err(err) => err.print().exit(),
    }

    save_edits(program, &input_path, new_file_path, delete_old);

    println!("Renamed program.");
}
