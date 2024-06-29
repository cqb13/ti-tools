pub mod txt_to_8xp;
pub mod xp_to_txt;

use std::io::Write;
use std::path::{Path, PathBuf};
use txt_to_8xp::convert_txt_to_8xp;
use xp_to_txt::convert_8xp_to_txt;

enum FileType {
    XP,
    TXT,
}

impl PartialEq for FileType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileType::XP, FileType::XP) => true,
            (FileType::TXT, FileType::TXT) => true,
            _ => false,
        }
    }
}

impl FileType {
    pub fn to_string(&self) -> String {
        match self {
            FileType::XP => "8xp",
            FileType::TXT => "txt",
        }
        .to_string()
    }

    pub fn opposite(&self) -> FileType {
        match self {
            FileType::XP => FileType::TXT,
            FileType::TXT => FileType::XP,
        }
    }
}

pub fn convert_command(
    input_path_string: String,
    output_path_string: Option<String>,
    name: Option<String>,
    raw: bool,
    display: bool,
) {
    let (input_path, output_path) = match confirm_paths(input_path_string, &output_path_string) {
        Ok((input_path, output_path)) => (input_path, output_path),
        Err(err) => {
            println!("{}", err);
            std::process::exit(0);
        }
    };

    let file_type = match get_conversion_file_type(input_path.as_path()) {
        Ok(file_type) => file_type,
        Err(err) => {
            println!("{}", err);
            std::process::exit(0)
        }
    };

    match file_type {
        FileType::XP => {
            let output: Vec<String> = convert_8xp_to_txt(input_path, raw, display);
            if output_path_string.is_none() {
                return;
            }

            if display || raw {
                println!();
            }

            write_to_file(&output_path, output.join(""), "8xp");
        }
        FileType::TXT => {
            if name.is_none() {
                println!("You must specify a name for the program");
                std::process::exit(0);
            }

            if name.as_ref().unwrap().len() > 8 {
                println!("Name must be 8 or less characters");
                std::process::exit(0);
            }

            if !name
                .as_ref()
                .unwrap()
                .chars()
                .all(|c| c.is_ascii_alphabetic())
            {
                println!("Name must be alphabetical characters only");
                std::process::exit(0);
            }

            let output: Vec<u8> =
                convert_txt_to_8xp(input_path, name.unwrap().to_string(), raw, display);
            if output_path_string.is_none() {
                return;
            }

            if display || raw {
                println!();
            }

            write_to_file(&output_path, output, "txt");
        }
    };
}

fn write_to_file<T: AsRef<[u8]>>(path: &Path, content: T, file_type: &str) {
    match std::fs::write(path, content) {
        Ok(_) => println!(
            "Successfully converted {} to {}",
            file_type,
            if file_type == "8xp" { "txt" } else { "8xp" }
        ),
        Err(err) => {
            eprintln!("Failed to write {}: {}", file_type, err);
            std::process::exit(1);
        }
    }
}

fn confirm_paths(
    input_path_string: String,
    output_path_string: &Option<String>,
) -> Result<(PathBuf, PathBuf), String> {
    let input_path = Path::new(&input_path_string);

    if !input_path.exists() {
        println!("No file is located at \"{}\"", input_path_string);
        std::process::exit(0);
    }

    let file_type = match get_conversion_file_type(input_path) {
        Ok(file_type) => file_type,
        Err(err) => return Err(err),
    };

    let output_path = match output_path_string {
        Some(output_path_string) => {
            match validate_and_fix_output_path(output_path_string.to_string(), file_type.opposite())
            {
                Ok(path_buf) => path_buf,
                Err(err) => return Err(err),
            }
        }
        None => PathBuf::new(),
    };

    Ok((input_path.to_path_buf(), output_path))
}

fn get_conversion_file_type(path: &Path) -> Result<FileType, String> {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("8xp") => Ok(FileType::XP),
            Some("txt") => Ok(FileType::TXT),
            _ => Err("Invalid file extension".to_string()),
        },
        None => Err("No file extension".to_string()),
    }
}

fn validate_and_fix_output_path(
    output_path_string: String,
    output_file_type: FileType,
) -> Result<PathBuf, String> {
    // We don't really care what the user put here, we already know the output type based on input
    // So if they messed up we can fix the extension for them
    let output_path_string = output_path_string.replace(
        &format!(".{}", output_file_type.opposite().to_string()),
        &format!(".{}", output_file_type.to_string()),
    );

    let output_path = Path::new(&output_path_string);

    if output_path.extension().is_none() {
        return Err("Output file must have an extension".to_string());
    }

    // just checking if valid file type
    match get_conversion_file_type(output_path) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    if output_path.exists() {
        println!("A file already exists at the output path, would you like to delete its content and proceed? [y/N]");
        let mut input = String::new();
        print!("> ");
        input.clear();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "y" || input == "Y" {
            println!("Deleting existing file");

            match std::fs::remove_file(output_path) {
                Ok(_) => {
                    println!("Deleted existing file");
                }
                Err(err) => {
                    return Err(format!("Failed to delete file: {}", err));
                }
            }
        } else {
            println!("Exiting due to existing output file");
            std::process::exit(0);
        }
    };

    Ok(output_path.to_owned())
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
