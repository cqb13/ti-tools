use std::path::Path;

pub fn rename_command(input_path_string: String, name: String, new_file: bool, delete_old: bool) {
    if name.len() > 8 {
        println!("Name must be 8 characters or less");
        std::process::exit(0);
    }

    if !name.chars().all(|c| c.is_ascii_alphabetic()) {
        println!("Name must be alphabetical characters only");
        std::process::exit(0);
    }

    let name = name.to_uppercase();

    let input_path = Path::new(&input_path_string);

    if !input_path.exists() {
        println!("No file is located at \"{}\"", input_path_string);
        std::process::exit(0);
    }

    match input_path.extension() {
        Some(ext) => match ext.to_str() {
            Some("8xp") => (),
            _ => {
                println!("Invalid file extension, must be 8xp");
                std::process::exit(0)
            }
        },
        None => {
            println!("No file extension");
            std::process::exit(0)
        }
    }

    let mut file = match std::fs::read(&input_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to read file: {}", err);
            std::process::exit(0);
        }
    };

    if file.len() < 76 {
        println!("8xp file is missing header and metadata");
        std::process::exit(0)
    }

    let (header_bytes, file) = file.split_at_mut(55);
    let (meta_data_bytes, file) = file.split_at_mut(19);

    let mut name_as_bytes = name.as_bytes().to_owned();

    while name_as_bytes.len() != 8 {
        name_as_bytes.push(0)
    }

    meta_data_bytes[5..13].copy_from_slice(&name_as_bytes);

    let mut file_with_renamed_program = Vec::new();
    file_with_renamed_program.extend_from_slice(header_bytes);
    file_with_renamed_program.extend_from_slice(meta_data_bytes);
    file_with_renamed_program.extend_from_slice(file);

    if new_file {
        let output_path = input_path.with_file_name(name).with_extension("8xp");

        match std::fs::write(&output_path, &file_with_renamed_program) {
            Ok(_) => {
                println!("Renamed program");
            }
            Err(err) => {
                println!("Failed to write to file: {}", err);
                std::process::exit(0);
            }
        }

        if delete_old {
            match std::fs::remove_file(input_path) {
                Ok(_) => {
                    println!("Deleted old file");
                }
                Err(err) => {
                    println!("Failed to delete old file: {}", err);
                    std::process::exit(0);
                }
            }
        }
    } else {
        match std::fs::write(input_path, &file_with_renamed_program) {
            Ok(_) => {
                println!("Renamed program")
            }
            Err(err) => {
                println!("Failed to write to file: {}", err);
                std::process::exit(0);
            }
        }
    }
}
