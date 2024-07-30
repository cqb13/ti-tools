use crate::calculator::program::{get_file_type, Program};
use crate::calculator::DisplayMode;
use crate::errors::CliError;
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
        Err(err) => err.print().exit(),
    };

    if !mass {
        let program = match decode_file(input_path, display_mode, content, preview) {
            Ok(program) => program,
            Err(err) => {
                err.print().exit();
            }
        };

        let name = program.metadata.name.to_string();

        if output_path_string.is_some() {
            match save_file(
                program,
                Path::new(&output_path_string.unwrap()),
                display_mode_string.as_str(),
            ) {
                Ok(_) => (),
                Err(err) => err.print().exit(),
            }
        }

        println!("Successfully converted {} to txt", name)
    } else {
        if !input_path.is_dir() {
            CliError::MassConversionInputNotDirectory("decoding".to_string())
                .print()
                .exit()
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
                if input != "y" && input != "Y" {
                    CliError::Quit("Missing output directory".to_string())
                        .print()
                        .exit()
                }

                println!("Creating output directory");
                fs::create_dir(Path::new(&output_path_string.as_ref().unwrap()))
                    .expect("Failed to create directory")
            }

            if !Path::new(&output_path_string.as_ref().unwrap()).is_dir() {
                CliError::MassConversionOutputNotDirectory("decoding".to_string())
                    .print()
                    .exit()
            }
        }

        for entry in fs::read_dir(input_path).expect("Failed to read directory") {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => {
                    CliError::FailedToReadDirectory(input_path_string.clone()).print();
                    continue;
                }
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

            let program = match decode_file(&path, display_mode.clone(), content, preview) {
                Ok(program) => program,
                Err(err) => {
                    println!("Failed to decode file:");
                    err.print();

                    println!("Would you like to skip this file and continue? [y/N]");
                    let mut input = String::new();
                    print!("> ");
                    input.clear();
                    std::io::stdout().flush().unwrap();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();
                    if input != "y" && input != "Y" {
                        CliError::Quit("User chose to quit".to_string())
                            .print()
                            .exit()
                    }

                    continue;
                }
            };

            let name = program.metadata.name.to_string();

            if output_path_string.is_some() {
                let output_path = Path::new(&output_path_string.as_ref().unwrap())
                    .join(&name)
                    .with_extension("txt");

                match save_file(program, output_path.as_path(), display_mode_string.as_str()) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("Failed to save file:");
                        err.print();

                        println!("Would you like to skip this file and continue? [y/N]");
                        let mut input = String::new();
                        print!("> ");
                        input.clear();
                        std::io::stdout().flush().unwrap();
                        std::io::stdin().read_line(&mut input).unwrap();
                        let input = input.trim();
                        if input != "y" && input != "Y" {
                            CliError::Quit("User chose to quit".to_string())
                                .print()
                                .exit()
                        }

                        continue;
                    }
                }
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
) -> Result<Program, CliError> {
    let program = Program::load_from_8xp(input_path.to_path_buf(), display_mode);

    let program = match program {
        Ok(program) => program,
        Err(err) => return Err(err),
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

    Ok(program)
}

fn save_file(
    program: Program,
    output_path: &Path,
    display_mode_string: &str,
) -> Result<(), CliError> {
    let result = program.save_to(output_path.to_path_buf());

    if display_mode_string == "pretty" {
        println!("Warning: Pretty tokens can't be accurately encoded")
    }

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
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
