use super::exit_with_error;
use crate::calculator::program::{get_file_type, Program};
use crate::calculator::DisplayMode;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn decode_command(
    input_path_string: String,
    output_path_string: Option<String>,
    display_mode_string: String,
    content: bool,
    preview: bool,
    mass: bool,
) {
    let input_path = Path::new(&input_path_string);

    let display_mode = match DisplayMode::from_string(&display_mode_string) {
        Ok(display_mode) => display_mode,
        Err(err) => exit_with_error(&err),
    };

    if !mass {
        let program = decode_file(input_path, display_mode, content, preview);

        let name = program.metadata.name.to_string();

        if output_path_string.is_some() {
            save_file(
                program,
                Path::new(&output_path_string.unwrap()),
                display_mode_string.as_str(),
            )
        }

        println!("Successfully converted {} to txt", name)
    } else {
        if !input_path.is_dir() {
            exit_with_error("When mass decoding the input path must lead to a directory")
        }

        if output_path_string.is_some() {
            if !Path::new(&output_path_string.as_ref().unwrap()).exists() {
                println!(
                    "The output directory does not exist. Would you like to create one? [y/N]"
                );
                let mut input = String::new();
                print!("> ");
                input.clear();
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                if input == "y" || input == "Y" {
                    println!("Creating output directory");
                    fs::create_dir(Path::new(&output_path_string.as_ref().unwrap()))
                        .expect("Failed to create directory")
                } else {
                    exit_with_error("Exiting due to missing output directory")
                }
            }

            if !Path::new(&output_path_string.as_ref().unwrap()).is_dir() {
                exit_with_error("When mass decoding the output path must lead to a directory")
            }
        }

        for entry in fs::read_dir(input_path).expect("Failed to read directory") {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => exit_with_error("Failed to read an entry in the input directory"),
            };

            let path = entry.path();
            match get_file_type(&path) {
                Ok(path_type) => {
                    if path_type.is_txt() {
                        continue;
                    }
                }
                Err(_) => continue,
            };

            let program = decode_file(&path, display_mode.clone(), content, preview);

            let name = program.metadata.name.to_string();

            if output_path_string.is_some() {
                let output_path = Path::new(&output_path_string.as_ref().unwrap())
                    .join(&name)
                    .with_extension("txt");

                save_file(program, output_path.as_path(), display_mode_string.as_str())
            }

            println!("Successfully converted {} to txt", name)
        }
    }
}

fn decode_file(
    input_path: &Path,
    display_mode: DisplayMode,
    content: bool,
    preview: bool,
) -> Program {
    let program = Program::load_from_8xp(input_path.to_path_buf(), display_mode);

    let program = match program {
        Ok(program) => program,
        Err(err) => exit_with_error(&err),
    };

    if content {
        let mut bytes: Vec<&u8> = Vec::new();
        bytes.extend(&program.header.bytes);
        bytes.extend(&program.metadata.bytes);
        bytes.extend(&program.body.bytes);
        bytes.extend(&program.checksum.bytes);

        print_bytes(bytes);
        println!();
    }

    if preview {
        println!("{}", program.display());
        println!();
    }

    program
}

fn save_file(program: Program, output_path: &Path, display_mode_string: &str) {
    let result = program.save_to(output_path.to_path_buf());

    if display_mode_string == "pretty" {
        println!("Warning: Pretty tokens can't be accurately encoded")
    }

    match result {
        Ok(_) => {}
        Err(err) => exit_with_error(&err),
    }
}

fn print_bytes(bytes: Vec<&u8>) {
    let mut i = 0;
    for byte in bytes {
        print!("{:02X}", byte);
        i += 1;
        if i % 16 == 0 {
            println!();
        } else {
            print!(", ");
        }
    }
}
