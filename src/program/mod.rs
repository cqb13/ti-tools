mod create;
mod decode;
mod encode;

use crate::tokens::OsVersion;
use create::create_from_8xp;
use std::io::Write;
use std::path::{Path, PathBuf};

pub enum DisplayMode {
    Pretty,
    Accessible,
    TiAscii,
}

pub struct Program {
    pub header: Header,
    pub metadata: Metadata,
    pub body: Body,
    pub checksum: Checksum,
}

impl Program {
    pub fn load(path: PathBuf, version: OsVersion) -> Result<Program, String> {
        if !path.exists() {
            return Err(format!(
                "Failed to find file at: {:?}",
                path.to_str().unwrap()
            ));
        }

        let file_type = match get_file_type(&path) {
            Ok(file_type) => file_type,
            Err(err) => return Err(err),
        };

        let (header, metadata, body, checksum) = match file_type {
            ProgramFileType::XP => create_from_8xp(&version, path),
            ProgramFileType::TXT => unimplemented!(),
        }
        .map_err(|err| err.to_string())?;

        let program = Program {
            header,
            metadata,
            body,
            checksum,
        };

        Ok(program)
    }

    pub fn save_to(&self, path: &str) -> Result<(), String> {
        let path = Path::new(path).to_path_buf();

        if path.exists() {
            println!("A file already exists at the output path, would you like to delete its content and proceed? [y/N]");
            let mut input = String::new();
            print!("> ");
            input.clear();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "y" || input == "Y" {
                println!("Deleting existing file");

                match std::fs::remove_file(&path) {
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
        }

        let file_type = match get_file_type(&path) {
            Ok(file_type) => file_type,
            Err(err) => return Err(err),
        };

        match file_type {
            ProgramFileType::XP => {
                let mut output_bytes = Vec::new();

                output_bytes.extend(self.header.bytes.to_vec());
                output_bytes.extend(self.metadata.bytes.to_vec());
                output_bytes.extend(self.body.bytes.to_vec());
                output_bytes.extend(self.checksum.bytes.to_vec());

                write_to_file(&path, output_bytes, "8xp");
            }
            ProgramFileType::TXT => {
                let output_string = format!(
                    "{}\n{}\n{}\n{}",
                    self.metadata.name,
                    self.metadata.file_type.to_string(),
                    self.metadata.archived.to_string(),
                    &self.body.translation
                );
                write_to_file(&path, output_string, "txt");
            }
        }

        Ok(())
    }

    pub fn display(&self) -> String {
        let header = self.header.display();
        let metadata = self.metadata.display();
        let body = self.body.display();
        let checksum = self.checksum.display();

        format!("{}\n{}\n{}\n{}", header, metadata, body, checksum)
    }
}

pub struct Header {
    pub bytes: Vec<u8>,
    pub signature: String,
    pub signature_extra: Vec<u8>,
    pub product_id: u8,
    pub comment: String,
    pub metadata_and_body_length: u16,
}

impl Header {
    fn new(
        bytes: Vec<u8>,
        signature: String,
        signature_extra: Vec<u8>,
        product_id: u8,
        comment: String,
        metadata_and_body_length: u16,
    ) -> Header {
        Header {
            bytes,
            signature,
            signature_extra,
            product_id,
            comment,
            metadata_and_body_length,
        }
    }

    pub fn display(&self) -> String {
        format!(
            "Header\n\
            Signature: {}\n\
            Signature Extra: {:02X?}\n\
            Product ID: {:02X?}\n\
            Comment: {}\n\
            Metadata and Body Length: {:04X?}",
            self.signature,
            self.signature_extra,
            self.product_id,
            self.comment,
            self.metadata_and_body_length
        )
    }
}

pub struct Metadata {
    pub bytes: Vec<u8>,
    pub flag: u8,
    pub unknown_byte: u8,
    pub body_and_checksum_length: u16,
    pub file_type: FileType,
    pub name: String,
    pub version: u8,
    pub archived: Archived,
    pub body_and_checksum_length_copy: u16,
    pub body_length: u16,
}

impl Metadata {
    fn new(
        bytes: Vec<u8>,
        flag: u8,
        unknown_byte: u8,
        body_and_checksum_length: u16,
        file_type: FileType,
        name: String,
        version: u8,
        archived: Archived,
        body_and_checksum_length_copy: u16,
        body_length: u16,
    ) -> Metadata {
        if body_and_checksum_length != body_and_checksum_length_copy {
            panic!(
                "body_and_checksum_length ({}) did not match body_and_checksum_length_copy ({})",
                body_and_checksum_length, body_and_checksum_length_copy
            )
        }

        if body_and_checksum_length - 2 != body_length {
            panic!("body_length ({}) does not match body_and_checksum_length ({}) after removing checksum", body_length, body_and_checksum_length)
        }

        Metadata {
            bytes,
            flag,
            unknown_byte,
            body_and_checksum_length,
            file_type,
            name,
            version,
            archived,
            body_and_checksum_length_copy,
            body_length,
        }
    }

    pub fn display(&self) -> String {
        format!(
            "Metadata\n\
            Flag: {:02X?}\n\
            Unknown Byte: {:02X?}\n\
            Body and Checksum Length: {:04X?}\n\
            File Type: {}\n\
            Name: {}\n\
            Version: {:02X?}\n\
            Archived: {}\n\
            Body and Checksum Length Copy: {:04X?}\n\
            Body Length: {:04X?}",
            self.flag,
            self.unknown_byte,
            self.body_and_checksum_length,
            self.file_type.to_string(),
            self.name,
            self.version,
            self.archived.to_string(),
            self.body_and_checksum_length_copy,
            self.body_length
        )
    }
}

pub struct Body {
    pub bytes: Vec<u8>,
    pub translation: String,
}

impl Body {
    fn new(bytes: Vec<u8>, translation: String) -> Body {
        Body { bytes, translation }
    }

    pub fn display(&self) -> String {
        format!(
            "Body\n\
            Translation:\n {}",
            self.translation
        )
    }
}

pub struct Checksum {
    pub bytes: Vec<u8>,
    pub value: u16,
}

impl Checksum {
    fn new(bytes: Vec<u8>, value: u16) -> Checksum {
        Checksum { bytes, value }
    }

    pub fn display(&self) -> String {
        format!(
            "Checksum\n\
            Value: {:04X?}",
            self.value
        )
    }
}

pub enum ProgramFileType {
    XP,
    TXT,
}

fn get_file_type(path: &PathBuf) -> Result<ProgramFileType, String> {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("8xp") => Ok(ProgramFileType::XP),
            Some("txt") => Ok(ProgramFileType::TXT),
            _ => Err("Invalid file extension".to_string()),
        },
        None => Err("No file extension".to_string()),
    }
}

pub enum FileType {
    Program,
    LockedProgram,
    Group,
    FlashApplication,
}

impl FileType {
    fn from_byte(byte: u8) -> FileType {
        match byte {
            0x05 => FileType::Program,
            0x06 => FileType::LockedProgram,
            0x17 => FileType::Group,
            0x24 => FileType::FlashApplication,
            _ => panic!("Unknown file type: {:02X?}", byte),
        }
    }

    fn to_string(&self) -> String {
        match self {
            FileType::Program => "Program".to_string(),
            FileType::LockedProgram => "Locked Program".to_string(),
            FileType::Group => "Group".to_string(),
            FileType::FlashApplication => "Flash Application".to_string(),
        }
    }
}

pub enum Archived {
    NotArchived,
    Archived,
}

impl Archived {
    fn from_byte(byte: u8) -> Archived {
        match byte {
            0x00 => Archived::NotArchived,
            0x80 => Archived::Archived,
            _ => panic!("Unknown archived byte: {:02X?}", byte),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Archived::NotArchived => "Not Archived".to_string(),
            Archived::Archived => "Archived".to_string(),
        }
    }
}

fn write_to_file<T: AsRef<[u8]>>(path: &PathBuf, content: T, file_type: &str) {
    match std::fs::write(path, content) {
        Ok(_) => println!("Successfully converted to {}", file_type),
        Err(err) => {
            println!("Failed to write {}: {}", file_type, err);
            std::process::exit(1);
        }
    }
}
