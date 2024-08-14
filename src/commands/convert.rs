use crate::calculator::errors::TiToolsError;
use crate::calculator::program::{get_file_type, Program, ProgramFileType};
use crate::calculator::{DisplayMode, EncodeMode};
use crate::prints;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn convert_command(
    input_path_string: String,
    output_path_string: Option<String>,
    display_mode_string: String,
    encode_mode: String,
    content: bool,
    preview: bool,
    mass: bool,
) {
    let input_path = Path::new(&input_path_string);

    // Validating input path
    if !input_path.exists() {
        prints!("[color:bright-red]Error:[color:reset] Failed to find file or directory at the input path");
        std::process::exit(1);
    }

    if mass && !input_path.is_dir() {
        prints!(
            "[color:bright-red]Error:[color:reset] When mass converting the input path must lead to a directory"
        );
        std::process::exit(1);
    }

    if !mass && input_path.is_dir() {
        prints!("[color:bright-red]Error:[color:reset] When converting the input path must lead to a file");
        std::process::exit(1);
    }

    // Validating output
    if output_path_string.is_none() && !preview && !content {
        prints!("[color:bright-yellow]Warning:[color:reset] An output path or preview option is required");
        std::process::exit(1);
    }

    // Normal conversion
    if !mass {
        let program_file_type = match get_file_type(&input_path.to_path_buf()) {
            Ok(program_file_type) => program_file_type,
            Err(err) => err.print().exit(),
        };

        // Validating args
        if program_file_type.is_8xp() && DisplayMode::from_string(&display_mode_string).is_err() {
            prints!("[color:bright-red]Error:[color:reset] Display mode is required for converting from 8xp files but could not be determined");
            std::process::exit(1);
        }

        if program_file_type.is_txt() && EncodeMode::from_string(&encode_mode).is_err() {
            prints!("[color:bright-red]Error:[color:reset] Encode mode is required for converting from 8xp files but could not be determined");
            std::process::exit(1);
        }

        let display_mode = match DisplayMode::from_string(&display_mode_string) {
            Ok(display_mode) => display_mode,
            Err(err) => err.print().exit(),
        };

        let encode_mode = match EncodeMode::from_string(&encode_mode) {
            Ok(encode_mode) => encode_mode,
            Err(err) => err.print().exit(),
        };

        let program = convert_program(
            &program_file_type,
            input_path,
            display_mode,
            encode_mode,
            content,
            preview,
        );

        if output_path_string.is_some() {
            match program.save_to(&Path::new(&output_path_string.as_ref().unwrap()).to_path_buf()) {
                Ok(_) => {
                    if display_mode_string == DisplayMode::Pretty.to_string() {
                        prints!(
                            "[color:bright-yellow]Warning:[color:reset] Pretty tokens can't be accurately encoded"
                        );
                    }

                    prints!(
                        "[color:bright-green]Successfully saved[color:reset] [color:bright-cyan]{}[color:reset] to [color:bright-cyan]{}",
                        program.metadata.name,
                        output_path_string.as_ref().unwrap()
                    );
                }
                Err(err) => err.print().exit(),
            }
        }

        return;
    }

    // Mass conversion
    if output_path_string.is_some() {
        if !Path::new(&output_path_string.as_ref().unwrap()).exists() {
            println!("The output directory does not exist. Would you like to create one? [y/N]");
            let mut input = String::new();
            print!("> ");
            input.clear();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input != "y" && input != "Y" {
                prints!("[color:bright-red]Error:[color:reset] An output directory must be created for files to be saved");
                std::process::exit(1)
            }

            println!("Creating output directory");
            fs::create_dir(Path::new(&output_path_string.as_ref().unwrap()))
                .expect("Failed to create directory")
        }
    }

    for entry in fs::read_dir(input_path).expect("Failed to read directory") {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                prints!("[color:bright-red]Error:[color:reset] Failed to read an entry in the input directory.");
                println!("Skipping entry...");
                continue;
            }
        };

        let path = entry.path();
        let file_type = match get_file_type(&path) {
            Ok(file_type) => file_type,
            Err(_) => continue,
        };

        // Validating args
        if file_type.is_8xp() && DisplayMode::from_string(&display_mode_string).is_err() {
            prints!("[color:bright-red]Error:[color:reset] Display mode is required for converting from 8xp files but could not be determined");
            std::process::exit(1);
        }

        if file_type.is_txt() && EncodeMode::from_string(&encode_mode).is_err() {
            prints!("[color:bright-red]Error:[color:reset] Encode mode is required for converting from 8xp files but could not be determined");
            std::process::exit(1);
        }

        let display_mode = match DisplayMode::from_string(&display_mode_string) {
            Ok(display_mode) => display_mode,
            Err(err) => err.print().exit(),
        };

        let encode_mode = match EncodeMode::from_string(&encode_mode) {
            Ok(encode_mode) => encode_mode,
            Err(err) => err.print().exit(),
        };

        let program = convert_program(
            &file_type,
            &path,
            display_mode,
            encode_mode,
            content,
            preview,
        );

        if output_path_string.is_some() {
            let name = program.metadata.name.to_string();

            match file_type {
                ProgramFileType::XP
                | ProgramFileType::XPThree
                | ProgramFileType::XPTwo
                | ProgramFileType::TXT => {
                    let output_path = if file_type.is_8xp() {
                        Path::new(&output_path_string.as_ref().unwrap())
                            .join(&name)
                            .with_extension("txt")
                    } else {
                        Path::new(&output_path_string.as_ref().unwrap())
                            .join(&name)
                            .with_extension("8xp")
                    };

                    match program.save_to(&output_path) {
                        Ok(_) => {
                            if display_mode_string == DisplayMode::Pretty.to_string() {
                                prints!(
                                "[color:bright-yellow]Warning:[color:reset] Pretty tokens can't be accurately encoded"
                            );
                            }

                            prints!(
                            "[color:bright-green]Successfully saved[color:reset] [color:bright-cyan]{}[color:reset] to [color:bright-cyan]{}.{}",
                            program.metadata.name,
                            output_path_string.as_ref().unwrap(),
                            output_path.extension().unwrap().to_str().unwrap()
                        );
                        }
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
                                TiToolsError::Quit("User chose to quit".to_string())
                                    .print()
                                    .exit()
                            }

                            continue;
                        }
                    }
                }
                ProgramFileType::JSON => {
                    let txt_output_path = Path::new(&output_path_string.as_ref().unwrap())
                        .join(&name)
                        .with_extension("txt");

                    let xp_output_path = Path::new(&output_path_string.as_ref().unwrap())
                        .join(&name)
                        .with_extension("8xp");

                    match program.save_to(&txt_output_path) {
                        Ok(_) => {
                            if display_mode_string == DisplayMode::Pretty.to_string() {
                                prints!(
                                    "[color:bright-yellow]Warning:[color:reset] Pretty tokens can't be accurately encoded"
                                );
                            }

                            prints!(
                                "[color:bright-green]Successfully saved[color:reset] [color:bright-cyan]{}[color:reset] to [color:bright-cyan]{}.txt",
                                program.metadata.name,
                                output_path_string.as_ref().unwrap()
                            );
                        }
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
                                TiToolsError::Quit("User chose to quit".to_string())
                                    .print()
                                    .exit()
                            }

                            continue;
                        }
                    }

                    match program.save_to(&xp_output_path) {
                        Ok(_) => {
                            if display_mode_string == DisplayMode::Pretty.to_string() {
                                prints!(
                                    "[color:bright-yellow]Warning:[color:reset] Pretty tokens can't be accurately encoded"
                                );
                            }

                            prints!(
                                "[color:bright-green]Successfully saved[color:reset] [color:bright-cyan]{}[color:reset] to [color:bright-cyan]{}.8xp",
                                program.metadata.name,
                                output_path_string.as_ref().unwrap()
                            );
                        }
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
                                TiToolsError::Quit("User chose to quit".to_string())
                                    .print()
                                    .exit()
                            }

                            continue;
                        }
                    }
                }
            }
        }
    }
}

fn convert_program(
    program_file_type: &ProgramFileType,
    input_path: &Path,
    display_mode: DisplayMode,
    encode_mode: EncodeMode,
    content: bool,
    preview: bool,
) -> Program {
    match program_file_type {
        ProgramFileType::XP | ProgramFileType::XPThree | ProgramFileType::XPTwo => {
            match Program::load_from_8xp(input_path.to_path_buf(), display_mode) {
                Ok(program) => {
                    if content {
                        let mut bytes: Vec<u8> = Vec::new();
                        bytes.extend(&program.header.bytes);
                        bytes.extend(&program.metadata.bytes);
                        bytes.extend(&program.body.bytes);
                        bytes.extend(&program.checksum.bytes);

                        print_bytes(bytes);
                        println!("\n");
                    }

                    if preview {
                        println!("{}\n", program.to_string());
                    }

                    program
                }
                Err(err) => err.print().exit(),
            }
        }
        ProgramFileType::TXT => {
            match Program::load_from_txt(input_path.to_path_buf(), &encode_mode) {
                Ok(program) => {
                    if content {
                        println!("{}\n", program.to_string());
                    }

                    if preview {
                        let mut bytes: Vec<u8> = Vec::new();
                        bytes.extend(&program.header.bytes);
                        bytes.extend(&program.metadata.bytes);
                        bytes.extend(&program.body.bytes);
                        bytes.extend(&program.checksum.bytes);

                        print_bytes(bytes);
                        println!("\n");
                    }

                    program
                }
                Err(err) => err.print().exit(),
            }
        }
        ProgramFileType::JSON => match Program::load_from_json(input_path.to_path_buf()) {
            Ok(program) => {
                if content {
                    let json_program = match serde_json::to_string_pretty(&program) {
                        Ok(json_program) => json_program,
                        Err(err) => {
                            prints!("[color:red]Error:[color:reset] Failed to convert program to json: {}", err);
                            std::process::exit(1);
                        }
                    };

                    println!("{}\n", json_program);
                }

                if preview {
                    let mut bytes: Vec<u8> = Vec::new();
                    bytes.extend(&program.header.bytes);
                    bytes.extend(&program.metadata.bytes);
                    bytes.extend(&program.body.bytes);
                    bytes.extend(&program.checksum.bytes);

                    print_bytes(bytes);
                    println!("\n");

                    println!("{}\n", program.to_string());
                }

                program
            }
            Err(err) => err.print().exit(),
        },
    }
}

fn print_bytes(bytes: Vec<u8>) {
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
