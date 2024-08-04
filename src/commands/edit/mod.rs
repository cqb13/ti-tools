pub mod archive;
pub mod comment;
pub mod lock;
pub mod rename;
pub mod unarchive;
pub mod unlock;

use crate::calculator::program::Program;
use crate::calculator::DisplayMode;
use crate::errors::CliError;
use std::path::{Path, PathBuf};

fn load_program(input_path: &PathBuf) -> Program {
    let program = Program::load_from_8xp(input_path.to_path_buf(), DisplayMode::Accessible);

    let program = match program {
        Ok(program) => program,
        Err(err) => err.print().exit(),
    };

    program
}

fn save_edits(
    program: Program,
    input_path: &PathBuf,
    new_file_path: Option<String>,
    delete_old: bool,
) {
    if new_file_path.is_none() {
        let result = program.save_to(&input_path.to_path_buf());

        match result {
            Ok(_) => {}
            Err(err) => err.print().exit(),
        }
    } else {
        let new_file_path = new_file_path.unwrap();
        let new_file_path = Path::new(&new_file_path).to_path_buf();
        let result = program.save_to(&new_file_path);

        match result {
            Ok(_) => {}
            Err(err) => err.print().exit(),
        }

        if delete_old {
            let result = std::fs::remove_file(input_path);

            match result {
                Ok(_) => {
                    println!("Deleted old file.");
                }
                Err(err) => CliError::FailedToDeleteFile(err.to_string()).print().exit(),
            }
        }
    }
}
