use crate::calculator::create::{create_from_8xp, create_from_txt};
use crate::calculator::{DisplayMode, EncodeMode};
use crate::tokens::OsVersion;
use std::io::Write;
use std::path::PathBuf;

pub struct Program {
    pub header: Header,
    pub metadata: Metadata,
    pub body: Body,
    pub checksum: Checksum,
    pub display_mode: DisplayMode,
}

impl Program {
    pub fn load_from_8xp(
        path: PathBuf,
        version: OsVersion,
        display_mode: DisplayMode,
    ) -> Result<Program, String> {
        if !path.exists() {
            return Err(format!(
                "Failed to find file at: {:?}",
                path.to_str().unwrap()
            ));
        }

        match get_file_type(&path) {
            Ok(file_type) => {
                if !file_type.is_8xp() {
                    return Err("File path must be to a .8xp file".to_string());
                }
            }
            Err(err) => return Err(err),
        };

        let (header, metadata, body, checksum) =
            match create_from_8xp(path, &version, &display_mode) {
                Ok((header, metadata, body, checksum)) => (header, metadata, body, checksum),
                Err(err) => return Err(err),
            };

        let program = Program {
            header,
            metadata,
            body,
            checksum,
            display_mode,
        };

        Ok(program)
    }

    pub fn load_from_txt(
        path: PathBuf,
        version: OsVersion,
        encode_mode: EncodeMode,
    ) -> Result<Program, String> {
        if !path.exists() {
            return Err(format!(
                "Failed to find file at: {:?}",
                path.to_str().unwrap()
            ));
        }

        match get_file_type(&path) {
            Ok(file_type) => {
                if !file_type.is_txt() {
                    return Err("File path must be to a .8xp file".to_string());
                }
            }
            Err(err) => return Err(err),
        };

        let (header, metadata, body, checksum) = match create_from_txt(path, &version, encode_mode)
        {
            Ok((header, metadata, body, checksum)) => (header, metadata, body, checksum),
            Err(err) => return Err(err),
        };

        let program = Program {
            header,
            metadata,
            body,
            checksum,
            display_mode: DisplayMode::Accessible,
        };

        Ok(program)
    }

    pub fn save_to(&self, path: PathBuf) -> Result<(), String> {
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
                return Err("Exiting due to existing output file".to_string());
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

                write_to_file(&path, output_bytes, "8xp")?
            }
            ProgramFileType::TXT => {
                let output_string = format!(
                    "{}\n{}\n{}\n{}\n{}",
                    self.metadata.name,
                    self.metadata.file_type.to_string(),
                    self.metadata.archived.to_string(),
                    self.display_mode.to_string(),
                    &self.body.translation
                );
                write_to_file(&path, output_string, "txt")?
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

    pub fn comment(&mut self, comment: String) -> Result<(), String> {
        if comment.len() > 42 {
            return Err("Comment must be 42 characters or less".to_string());
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
    pub archived: Archived,
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
        archived: Archived,
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
            Body and Checksum Length: {}\n\
            File Type: {}\n\
            Name: {}\n\
            Version: {:02X?}\n\
            Archived: {}\n\
            Body and Checksum Length Copy: {}\n\
            Body Length: {}",
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

    pub fn rename(&mut self, name: String) -> Result<(), String> {
        if name.len() > 8 {
            return Err("Name must be 8 characters or less".to_string());
        }

        if !name.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err("Name must be alphabetical characters only".to_string());
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
    pub fn lock(&mut self) -> Result<(), String> {
        self.bytes[5] = FileType::LockedProgram.to_byte();
        self.file_type = FileType::LockedProgram;

        Ok(())
    }

    pub fn unlock(&mut self) -> Result<(), String> {
        self.bytes[5] = FileType::Program.to_byte();
        self.file_type = FileType::Program;

        Ok(())
    }

    // byte 15 is the archived byte
    pub fn archive(&mut self) -> Result<(), String> {
        self.bytes[15] = Archived::Archived.to_byte();
        self.archived = Archived::Archived;

        Ok(())
    }

    pub fn unarchive(&mut self) -> Result<(), String> {
        self.bytes[15] = Archived::NotArchived.to_byte();
        self.archived = Archived::NotArchived;

        Ok(())
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
}

impl ProgramFileType {
    pub fn is_8xp(&self) -> bool {
        match self {
            ProgramFileType::XP => true,
            ProgramFileType::TXT => false,
        }
    }

    pub fn is_txt(&self) -> bool {
        match self {
            ProgramFileType::XP => false,
            ProgramFileType::TXT => true,
        }
    }
}

pub fn get_file_type(path: &PathBuf) -> Result<ProgramFileType, String> {
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
    pub fn from_byte(byte: u8) -> Result<FileType, String> {
        match byte {
            0x05 => Ok(FileType::Program),
            0x06 => Ok(FileType::LockedProgram),
            0x17 => Ok(FileType::Group),
            0x24 => Ok(FileType::FlashApplication),
            _ => Err(format!("Unknown file type byte: {:02X?}", byte)),
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

    pub fn from_string(file_type: &str) -> Result<FileType, String> {
        match file_type {
            "Program" => Ok(FileType::Program),
            "Locked Program" => Ok(FileType::LockedProgram),
            "Group" => Ok(FileType::Group),
            "Flash Application" => Ok(FileType::FlashApplication),
            _ => Err(format!("Unknown file type string: {}", file_type)),
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

pub enum Archived {
    NotArchived,
    Archived,
}

impl Archived {
    pub fn from_byte(byte: u8) -> Result<Archived, String> {
        match byte {
            0x00 => Ok(Archived::NotArchived),
            0x80 => Ok(Archived::Archived),
            _ => Err(format!("Unknown archived byte: {:02X?}", byte)),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            Archived::NotArchived => 0x00,
            Archived::Archived => 0x80,
        }
    }

    pub fn from_string(archived: &str) -> Result<Archived, String> {
        match archived {
            "Not Archived" => Ok(Archived::NotArchived),
            "Archived" => Ok(Archived::Archived),
            _ => Err(format!("Unknown archived string: {}", archived)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Archived::NotArchived => "Not Archived".to_string(),
            Archived::Archived => "Archived".to_string(),
        }
    }
}

fn write_to_file<T: AsRef<[u8]>>(
    path: &PathBuf,
    content: T,
    file_type: &str,
) -> Result<(), String> {
    match std::fs::write(path, content) {
        Ok(_) => println!("Successfully saved to {}", path.display()),
        Err(err) => return Err(format!("Failed to write {} file: {}", file_type, err)),
    }

    Ok(())
}