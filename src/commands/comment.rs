use super::exit_with_error;
use crate::program::{DisplayMode, Model, Program};
use crate::tokens::OsVersion;
use std::path::Path;

pub fn comment_command(
    input_path_string: String,
    comment: String,
    model: String,
    new_file_path: Option<String>,
    delete_old: bool,
) {
    let model = match Model::from_string(&model) {
        Ok(model) => model,
        Err(err) => exit_with_error(&err),
    };

    let target_version = OsVersion {
        model,
        version: "latest".to_string(),
    };

    let input_path = Path::new(&input_path_string).to_path_buf();

    let program = Program::load_from_8xp(
        input_path.to_path_buf(),
        target_version,
        DisplayMode::Accessible,
    );

    let mut program = match program {
        Ok(program) => program,
        Err(err) => exit_with_error(&err),
    };

    let result = program.header.comment(comment);

    match result {
        Ok(_) => {}
        Err(err) => exit_with_error(&err),
    }

    if new_file_path.is_none() {
        let result = program.save_to(input_path.to_path_buf());

        match result {
            Ok(_) => {}
            Err(err) => exit_with_error(&err),
        }
    } else {
        let new_file_path = new_file_path.unwrap();
        let new_file_path = Path::new(&new_file_path).to_path_buf();
        let result = program.save_to(new_file_path);

        match result {
            Ok(_) => {}
            Err(err) => exit_with_error(&err),
        }

        if delete_old {
            let result = std::fs::remove_file(input_path);

            match result {
                Ok(_) => {
                    println!("Deleted old file.");
                }
                Err(err) => exit_with_error(&err.to_string()),
            }
        }
    }
}
