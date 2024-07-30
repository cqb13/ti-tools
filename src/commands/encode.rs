use crate::calculator::program::{get_file_type, Program};
use crate::calculator::EncodeMode;
use crate::errors::CliError;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn encode_command(
    input_path_string: String,
    output_path_string: Option<String>,
    encode_mode: String,
    content: bool,
    preview: bool,
    mass: bool,
) {
    let encode_mode = match EncodeMode::from_string(&encode_mode) {
        Ok(encode_mode) => encode_mode,
        Err(err) => err.print().exit(),
    };

    let input_path = Path::new(&input_path_string);

    if !mass {
        let program = match encode_file(&input_path, &encode_mode, content, preview) {
            Ok(program) => program,
            Err(err) => {
                err.print().exit();
            }
        };

        let name = program.metadata.name.to_string();

        if output_path_string.is_some() {
            match save_file(program, Path::new(&output_path_string.unwrap())) {
                Ok(_) => (),
                Err(err) => err.print().exit(),
            }
        }

        println!("Successfully converted {} to 8xp", name)
    } else {
        if !input_path.is_dir() {
            CliError::MassConversionInputNotDirectory("encoding".to_string())
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
                    println!("{}", input);
                    CliError::Quit("Missing output directory".to_string())
                        .print()
                        .exit()
                }

                println!("Creating output directory");
                fs::create_dir(Path::new(&output_path_string.as_ref().unwrap()))
                    .expect("Failed to create directory")
            }

            if !Path::new(&output_path_string.as_ref().unwrap()).is_dir() {
                CliError::MassConversionOutputNotDirectory("encoding".to_string())
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
                    if path_type.is_8xp() {
                        continue;
                    }
                }
                Err(_) => continue,
            };

            let program = match encode_file(&path, &encode_mode, content, preview) {
                Ok(program) => program,
                Err(err) => {
                    println!("Failed to encode file:");
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
                    .with_extension("8xp");

                match save_file(program, output_path.as_path()) {
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

fn encode_file(
    input_path: &Path,
    encode_mode: &EncodeMode,
    content: bool,
    preview: bool,
) -> Result<Program, CliError> {
    let program = Program::load_from_txt(input_path.to_path_buf(), encode_mode);

    let program = match program {
        Ok(program) => program,
        Err(err) => return Err(err),
    };

    if content {
        println!("{}", program.display());
        println!();
    }

    if preview {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(&program.header.bytes);
        bytes.extend(&program.metadata.bytes);
        bytes.extend(&program.body.bytes);
        bytes.extend(&program.checksum.bytes);

        print_bytes(&bytes);
        println!();
    }

    Ok(program)
}

fn save_file(program: Program, output_path: &Path) -> Result<(), CliError> {
    let result = program.save_to(output_path.to_path_buf());

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

fn print_bytes(file: &Vec<u8>) {
    let mut i = 0;
    for byte in file {
        print!("{:02X}", byte);
        i += 1;
        if i % 16 == 0 {
            println!();
        } else {
            print!(", ");
        }
    }
}
