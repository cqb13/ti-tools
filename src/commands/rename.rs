use crate::program::{DisplayMode, Model, Program};
use crate::tokens::OsVersion;
use std::path::Path;

pub fn rename_command(
    input_path_string: String,
    name: String,
    new_file_path: Option<String>,
    delete_old: bool,
) {
    let target_version = OsVersion {
        model: Model::Latest,
        version: "latest".to_string(),
    };

    let input_path = Path::new(&input_path_string).to_path_buf();

    if !input_path.exists() {
        println!("File does not exist.");
        std::process::exit(0);
    }

    let program = Program::load_from_8xp(
        input_path.to_path_buf(),
        target_version,
        DisplayMode::Accessible,
    );

    let mut program = match program {
        Ok(program) => program,
        Err(err) => {
            println!("{}", err);
            std::process::exit(0);
        }
    };

    let result = program.metadata.rename(name);

    match result {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
            std::process::exit(0);
        }
    }

    if new_file_path.is_none() {
        let result = program.save_to(input_path.to_path_buf());

        match result {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
                std::process::exit(0);
            }
        }
    } else {
        let new_file_path = new_file_path.unwrap();
        let new_file_path = Path::new(&new_file_path).to_path_buf();
        let result = program.save_to(new_file_path);

        match result {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
                std::process::exit(0);
            }
        }

        if delete_old {
            let result = std::fs::remove_file(input_path);

            match result {
                Ok(_) => {
                    println!("Deleted old file.");
                }
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(0);
                }
            }
        }
    }
}
