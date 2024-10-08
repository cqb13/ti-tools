use super::models::ModelDetails;
use crate::calculator::errors::TiToolsError;
use crate::calculator::file::from_8xp::create_from_8xp;
use crate::calculator::file::from_txt::create_from_txt;
use crate::calculator::{DisplayMode, EncodeMode};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub header: Header,
    pub metadata: Metadata,
    pub body: Body,
    pub checksum: Checksum,
    pub display_mode: DisplayMode,
    pub model: ModelDetails,
}

impl Program {
    pub fn load_from_8xp(
        path: PathBuf,
        display_mode: DisplayMode,
    ) -> Result<Program, TiToolsError> {
        if !path.exists() {
            return Err(TiToolsError::FailedToFindFile(
                path.to_str().unwrap().to_string(),
            ));
        }

        let file_type = match get_file_type(&path) {
            Ok(file_type) => {
                if !file_type.is_8xp() {
                    return Err(TiToolsError::IncompatibleFileType(
                        "txt/json".to_string(),
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

    pub fn load_from_txt(path: PathBuf, encode_mode: &EncodeMode) -> Result<Program, TiToolsError> {
        if !path.exists() {
            return Err(TiToolsError::FailedToFindFile(
                path.to_str().unwrap().to_string(),
            ));
        }

        match get_file_type(&path) {
            Ok(file_type) => {
                if !file_type.is_txt() {
                    return Err(TiToolsError::IncompatibleFileType(
                        "8xp/83p/82p".to_string(),
                        "txt or json".to_string(),
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

    pub fn load_from_json(path: PathBuf) -> Result<Program, TiToolsError> {
        if !path.exists() {
            return Err(TiToolsError::FailedToFindFile(
                path.to_str().unwrap().to_string(),
            ));
        }

        match get_file_type(&path) {
            Ok(file_type) => {
                if !file_type.is_json() {
                    return Err(TiToolsError::IncompatibleFileType(
                        "json".to_string(),
                        "json".to_string(),
                    ));
                }
            }
            Err(err) => return Err(err),
        };

        let file_string = match std::fs::read_to_string(&path) {
            Ok(file_string) => file_string,
            Err(err) => return Err(TiToolsError::FailedToReadFile(err.to_string())),
        };

        let program: Program = match serde_json::from_str(&file_string) {
            Ok(program) => program,
            Err(err) => return Err(TiToolsError::FailedToDeserializeJson(err.to_string())),
        };

        Ok(program)
    }

    pub fn save_to(&self, path: &PathBuf) -> Result<(), TiToolsError> {
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

                match std::fs::remove_file(path) {
                    Ok(_) => {
                        println!("Deleted existing file");
                    }
                    Err(err) => {
                        return Err(TiToolsError::FailedToDeleteFile(err.to_string()));
                    }
                }
            } else {
                return Err(TiToolsError::Quit(
                    "User chose not to overwrite file".to_string(),
                ));
            }
        }

        let file_type = match get_file_type(path) {
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

                write_to_file(path, output_bytes)?
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
                write_to_file(path, output_string)?
            }
            ProgramFileType::JSON => {
                let output_string = match serde_json::to_string_pretty(&self) {
                    Ok(output_string) => output_string,
                    Err(err) => {
                        return Err(TiToolsError::FailedToSerializeJson(err.to_string()));
                    }
                };

                write_to_file(path, output_string)?
            }
        }

        Ok(())
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}\n{}\nTotal Size: {} bytes\nBody Size: {} bytes\n----- Status -----\nmodel: {}\nlanguage: {}\nDestination: {}\nFile Type: {}\n----- Program -----\n{}",
            self.metadata.name,
            self.header.comment,
            self.header.bytes.len() + self.metadata.bytes.len() + self.body.bytes.len() + 2,
            self.body.bytes.len(),
            self.model.model.to_string(),
            self.model.language,
            self.metadata.destination.to_string(),
            self.metadata.file_type.to_string(),
            self.body.translation
        )
    }
}

#[derive(Serialize, Deserialize)]
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

    pub fn comment(&mut self, comment: String) -> Result<(), TiToolsError> {
        if comment.len() > 42 {
            return Err(TiToolsError::InvalidCommentLength);
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

#[derive(Serialize, Deserialize)]
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

    pub fn rename(&mut self, name: String) -> Result<(), TiToolsError> {
        if name.len() > 8 {
            return Err(TiToolsError::InvalidNameLength);
        }

        if !name.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(TiToolsError::InvalidNameCharacters);
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum ProgramFileType {
    XP,
    TXT,
    XPTwo,
    XPThree,
    JSON,
}

impl ProgramFileType {
    pub fn is_8xp(&self) -> bool {
        !matches!(self, ProgramFileType::TXT)
    }

    pub fn is_txt(&self) -> bool {
        matches!(self, ProgramFileType::TXT)
    }

    pub fn is_json(&self) -> bool {
        matches!(self, ProgramFileType::JSON)
    }

    pub fn to_string(&self) -> String {
        match self {
            ProgramFileType::XP => "8xp".to_string(),
            ProgramFileType::TXT => "txt".to_string(),
            ProgramFileType::XPTwo => "82p".to_string(),
            ProgramFileType::XPThree => "83p".to_string(),
            ProgramFileType::JSON => "json".to_string(),
        }
    }
}

pub fn get_file_type(path: &Path) -> Result<ProgramFileType, TiToolsError> {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("8xp") => Ok(ProgramFileType::XP),
            Some("83p") => Ok(ProgramFileType::XPThree),
            Some("82p") => Ok(ProgramFileType::XPTwo),
            Some("txt") => Ok(ProgramFileType::TXT),
            Some("json") => Ok(ProgramFileType::JSON),
            _ => Err(TiToolsError::InvalidExtension(
                ext.to_str().unwrap().to_string(),
            )),
        },
        None => Err(TiToolsError::MissingExtension),
    }
}

#[derive(Serialize, Deserialize)]
pub enum FileType {
    Program,
    LockedProgram,
    Group,
    FlashApplication,
}

impl FileType {
    pub fn from_byte(byte: u8) -> Result<FileType, TiToolsError> {
        match byte {
            0x05 => Ok(FileType::Program),
            0x06 => Ok(FileType::LockedProgram),
            0x17 => Ok(FileType::Group),
            0x24 => Ok(FileType::FlashApplication),
            _ => Err(TiToolsError::Match(
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

    pub fn from_string(file_type: &str) -> Result<FileType, TiToolsError> {
        match file_type {
            "Program" => Ok(FileType::Program),
            "Locked Program" => Ok(FileType::LockedProgram),
            "Group" => Ok(FileType::Group),
            "Flash Application" => Ok(FileType::FlashApplication),
            _ => Err(TiToolsError::Match(
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

#[derive(Serialize, Deserialize)]
pub enum Destination {
    RAM,
    Archive,
}

impl Destination {
    pub fn from_byte(byte: u8) -> Result<Destination, TiToolsError> {
        match byte {
            0x00 => Ok(Destination::RAM),
            0x80 => Ok(Destination::Archive),
            _ => Err(TiToolsError::Match(
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

    pub fn from_string(archived: &str) -> Result<Destination, TiToolsError> {
        match archived {
            "RAM" => Ok(Destination::RAM),
            "Archive" => Ok(Destination::Archive),
            _ => Err(TiToolsError::Match(
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

fn write_to_file<T: AsRef<[u8]>>(path: &PathBuf, content: T) -> Result<(), TiToolsError> {
    match std::fs::write(path, content) {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err(TiToolsError::FailedToWriteFile(
                path.to_str().unwrap().to_string(),
                err.to_string(),
            ))
        }
    }
}
