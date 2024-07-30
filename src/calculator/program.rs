use super::models::ModelDetails;
use crate::calculator::create::from_8xp::create_from_8xp;
use crate::calculator::create::from_txt::create_from_txt;
use crate::calculator::{DisplayMode, EncodeMode};
use crate::errors::CliError;
use std::io::Write;
use std::path::PathBuf;

pub struct Program {
    pub header: Header,
    pub metadata: Metadata,
    pub body: Body,
    pub checksum: Checksum,
    pub display_mode: DisplayMode,
    pub model: ModelDetails,
}

impl Program {
    pub fn load_from_8xp(path: PathBuf, display_mode: DisplayMode) -> Result<Program, CliError> {
        if !path.exists() {
            return Err(CliError::FailedToFindFile(
                path.to_str().unwrap().to_string(),
            ));
        }

        let file_type = match get_file_type(&path) {
            Ok(file_type) => {
                if !file_type.is_8xp() {
                    return Err(CliError::IncompatibleFileType(
                        "txt".to_string(),
                        "8xp/83p/82p".to_string(),
                    ));
                }
                file_type
            }
            Err(err) => return Err(err),
        };

        let (header, metadata, body, checksum, model) =
            match create_from_8xp(path, file_type, &display_mode) {
                Ok((header, metadata, body, checksum, model)) => {
                    (header, metadata, body, checksum, model)
                }
                Err(err) => return Err(err),
            };

        let program = Program {
            header,
            metadata,
            body,
            checksum,
            display_mode,
            model,
        };

        Ok(program)
    }

    pub fn load_from_txt(path: PathBuf, encode_mode: &EncodeMode) -> Result<Program, CliError> {
        if !path.exists() {
            return Err(CliError::FailedToFindFile(
                path.to_str().unwrap().to_string(),
            ));
        }

        match get_file_type(&path) {
            Ok(file_type) => {
                if !file_type.is_txt() {
                    return Err(CliError::IncompatibleFileType(
                        "8xp/83p/82p".to_string(),
                        "txt".to_string(),
                    ));
                }
            }
            Err(err) => return Err(err),
        };

        let (header, metadata, body, checksum, model) = match create_from_txt(path, encode_mode) {
            Ok((header, metadata, body, checksum, model)) => {
                (header, metadata, body, checksum, model)
            }
            Err(err) => return Err(err),
        };

        let program = Program {
            header,
            metadata,
            body,
            checksum,
            display_mode: DisplayMode::Accessible,
            model,
        };

        Ok(program)
    }

    pub fn save_to(&self, path: PathBuf) -> Result<(), CliError> {
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
                        return Err(CliError::FailedToDeleteFile(err.to_string()));
                    }
                }
            } else {
                return Err(CliError::Quit(
                    "User chose not to overwrite file".to_string(),
                ));
            }
        }

        let file_type = match get_file_type(&path) {
            Ok(file_type) => file_type,
            Err(err) => return Err(err),
        };

        match file_type {
            ProgramFileType::XP | ProgramFileType::XPThree | ProgramFileType::XPTwo => {
                let mut output_bytes = Vec::new();

                output_bytes.extend(self.header.bytes.to_vec());
                output_bytes.extend(self.metadata.bytes.to_vec());
                output_bytes.extend(self.body.bytes.to_vec());
                output_bytes.extend(self.checksum.bytes.to_vec());

                write_to_file(&path, output_bytes)?
            }
            ProgramFileType::TXT => {
                let output_string = format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}",
                    self.metadata.name,
                    self.header.comment,
                    self.metadata.file_type.to_string(),
                    self.metadata.destination.to_string(),
                    self.display_mode.to_string(),
                    self.model.model.to_string(),
                    &self.body.translation
                );
                write_to_file(&path, output_string)?
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
    pub fn new(
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
            Metadata and Body Length: {}",
            self.signature,
            self.signature_extra,
            self.product_id,
            self.comment,
            self.metadata_and_body_length
        )
    }

    pub fn comment(&mut self, comment: String) -> Result<(), CliError> {
        if comment.len() > 42 {
            return Err(CliError::InvalidCommentLength);
        }

        let mut comment_bytes = comment.as_bytes().to_vec();
        while comment_bytes.len() != 42 {
            comment_bytes.push(0x00)
        }

        // name is in bytes 11-53
        self.bytes.splice(11..53, comment_bytes.iter().cloned());
        self.comment = comment;

        Ok(())
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
    pub destination: Destination,
    pub body_and_checksum_length_copy: u16,
    pub body_length: u16,
}

impl Metadata {
    pub fn new(
        bytes: Vec<u8>,
        flag: u8,
        unknown_byte: u8,
        body_and_checksum_length: u16,
        file_type: FileType,
        name: String,
        version: u8,
        destination: Destination,
        body_and_checksum_length_copy: u16,
        body_length: u16,
    ) -> Metadata {
        Metadata {
            bytes,
            flag,
            unknown_byte,
            body_and_checksum_length,
            file_type,
            name,
            version,
            destination,
            body_and_checksum_length_copy,
            body_length,
        }
    }

    pub fn display(&self) -> String {
        format!(
            "Metadata\n\
            Flag: {:02X?}\n\
            Unknown Byte: {:02X?}\n\
            Body and Checksum Length: {}\n\
            File Type: {}\n\
            Name: {}\n\
            Version: {:02X?}\n\
            Destination: {}\n\
            Body and Checksum Length Copy: {}\n\
            Body Length: {}",
            self.flag,
            self.unknown_byte,
            self.body_and_checksum_length,
            self.file_type.to_string(),
            self.name,
            self.version,
            self.destination.to_string(),
            self.body_and_checksum_length_copy,
            self.body_length
        )
    }

    pub fn rename(&mut self, name: String) -> Result<(), CliError> {
        if name.len() > 8 {
            return Err(CliError::InvalidNameLength);
        }

        if !name.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(CliError::InvalidNameCharacters);
        }

        let name = name.to_uppercase();

        let mut name_bytes = name.as_bytes().to_vec();
        while name_bytes.len() != 8 {
            name_bytes.push(0x00)
        }

        // name is in bytes 5-13
        self.bytes.splice(5..13, name_bytes.iter().cloned());
        self.name = name;

        Ok(())
    }

    // byte 5 is the file type byte
    pub fn lock(&mut self) {
        self.bytes[4] = FileType::LockedProgram.to_byte();
        self.file_type = FileType::LockedProgram;
    }

    pub fn unlock(&mut self) {
        self.bytes[4] = FileType::Program.to_byte();
        self.file_type = FileType::Program;
    }

    // byte 15 is the archived byte
    pub fn archive(&mut self) {
        self.bytes[14] = Destination::Archive.to_byte();
        self.destination = Destination::Archive;
    }

    pub fn unarchive(&mut self) {
        self.bytes[14] = Destination::RAM.to_byte();
        self.destination = Destination::RAM;
    }
}

pub struct Body {
    pub bytes: Vec<u8>,
    pub translation: String,
}

impl Body {
    pub fn new(bytes: Vec<u8>, translation: String) -> Body {
        Body { bytes, translation }
    }

    pub fn display(&self) -> String {
        format!(
            "Body\n\
            Translation:\n{}",
            self.translation
        )
    }
}

pub struct Checksum {
    pub bytes: Vec<u8>,
    pub value: u16,
}

impl Checksum {
    pub fn new(bytes: Vec<u8>, value: u16) -> Checksum {
        Checksum { bytes, value }
    }

    pub fn display(&self) -> String {
        format!(
            "Checksum\n\
            Value: {}",
            self.value
        )
    }
}

pub enum ProgramFileType {
    XP,
    TXT,
    XPTwo,
    XPThree,
}

impl ProgramFileType {
    pub fn is_8xp(&self) -> bool {
        match self {
            ProgramFileType::TXT => false,
            _ => true,
        }
    }

    pub fn is_txt(&self) -> bool {
        match self {
            ProgramFileType::TXT => true,
            _ => false,
        }
    }
}

pub fn get_file_type(path: &PathBuf) -> Result<ProgramFileType, CliError> {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("8xp") => Ok(ProgramFileType::XP),
            Some("83p") => Ok(ProgramFileType::XPThree),
            Some("82p") => Ok(ProgramFileType::XPTwo),
            Some("txt") => Ok(ProgramFileType::TXT),
            _ => Err(CliError::InvalidExtension(
                ext.to_str().unwrap().to_string(),
            )),
        },
        None => Err(CliError::MissingExtension),
    }
}

pub enum FileType {
    Program,
    LockedProgram,
    Group,
    FlashApplication,
}

impl FileType {
    pub fn from_byte(byte: u8) -> Result<FileType, CliError> {
        match byte {
            0x05 => Ok(FileType::Program),
            0x06 => Ok(FileType::LockedProgram),
            0x17 => Ok(FileType::Group),
            0x24 => Ok(FileType::FlashApplication),
            _ => Err(CliError::Match(
                format!("{:02X}", byte),
                "File Type".to_string(),
            )),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            FileType::Program => 0x05,
            FileType::LockedProgram => 0x06,
            FileType::Group => 0x17,
            FileType::FlashApplication => 0x24,
        }
    }

    pub fn from_string(file_type: &str) -> Result<FileType, CliError> {
        match file_type {
            "Program" => Ok(FileType::Program),
            "Locked Program" => Ok(FileType::LockedProgram),
            "Group" => Ok(FileType::Group),
            "Flash Application" => Ok(FileType::FlashApplication),
            _ => Err(CliError::Match(
                file_type.to_string(),
                "File Type".to_string(),
            )),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FileType::Program => "Program".to_string(),
            FileType::LockedProgram => "Locked Program".to_string(),
            FileType::Group => "Group".to_string(),
            FileType::FlashApplication => "Flash Application".to_string(),
        }
    }
}

pub enum Destination {
    RAM,
    Archive,
}

impl Destination {
    pub fn from_byte(byte: u8) -> Result<Destination, CliError> {
        match byte {
            0x00 => Ok(Destination::RAM),
            0x80 => Ok(Destination::Archive),
            _ => Err(CliError::Match(
                format!("{:02X}", byte),
                "Destination".to_string(),
            )),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            Destination::RAM => 0x00,
            Destination::Archive => 0x80,
        }
    }

    pub fn from_string(archived: &str) -> Result<Destination, CliError> {
        match archived {
            "RAM" => Ok(Destination::RAM),
            "Archive" => Ok(Destination::Archive),
            _ => Err(CliError::Match(
                archived.to_string(),
                "Destination".to_string(),
            )),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Destination::RAM => "RAM".to_string(),
            Destination::Archive => "Archive".to_string(),
        }
    }
}

fn write_to_file<T: AsRef<[u8]>>(path: &PathBuf, content: T) -> Result<(), CliError> {
    match std::fs::write(path, content) {
        Ok(_) => println!("Successfully saved to {}", path.display()),
        Err(err) => {
            return Err(CliError::FailedToWriteFile(
                path.to_str().unwrap().to_string(),
                err.to_string(),
            ))
        }
    }

    Ok(())
}
