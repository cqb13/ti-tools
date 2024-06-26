pub mod xp_to_txt;

use std::io::Write;
use std::path::{Path, PathBuf};
use xp_to_txt::convert_8xp_to_txt;

pub enum FileType {
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
    header: bool,
    metadata: bool,
    checksum: bool,
    bytes: bool,
    display: bool,
    log_messages: bool,
) {
    let (input_path, output_path) =
        match confirm_paths(input_path_string, &output_path_string, log_messages) {
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

    let output_file = match file_type {
        FileType::XP => convert_8xp_to_txt(
            input_path,
            header,
            metadata,
            checksum,
            bytes,
            display,
            log_messages,
        ),
        FileType::TXT => convert_txt_to_8xp(input_path, bytes),
    };

    if output_path_string.is_none() {
        return;
    }

    if display || bytes {
        println!();
    }

    if log_messages {
        println!("Writing to file");
    }

    match std::fs::write(output_path, output_file.join("")) {
        Ok(_) => {
            if log_messages {
                println!("Wrote to file");
            }
        }
        Err(err) => {
            println!("Failed to write to file: {}", err);
            std::process::exit(0);
        }
    }
}

fn convert_txt_to_8xp(input_path: PathBuf, bytes: bool) -> Vec<String> {
    Vec::new()
}

pub fn print_bytes(file: &Vec<u8>) {
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

fn confirm_paths(
    input_path_string: String,
    output_path_string: &Option<String>,
    log_messages: bool,
) -> Result<(PathBuf, PathBuf), String> {
    let input_path = Path::new(&input_path_string);

    if !input_path.exists() {
        panic!(
            "No file is located at \"{}\", please provide a valid path",
            input_path_string
        );
    }

    let file_type = match get_conversion_file_type(input_path) {
        Ok(file_type) => file_type,
        Err(err) => return Err(err),
    };

    let output_path = match output_path_string {
        Some(output_path_string) => {
            match validate_and_fix_output_path(
                output_path_string.to_string(),
                file_type.opposite(),
                log_messages,
            ) {
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
    log_messages: bool,
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
            if log_messages {
                println!("Deleting existing file");
            }
            match std::fs::remove_file(output_path) {
                Ok(_) => {
                    if log_messages {
                        println!("Deleted existing file");
                    }
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
