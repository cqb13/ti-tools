mod create;
mod decode;
mod encode;

use crate::tokens::OsVersion;
use create::{create_from_8xp, create_from_txt};
use std::fmt::Debug;
use std::io::Write;
use std::path::PathBuf;

pub enum EncodeMode {
    Min,
    Max,
    Smart,
}

impl EncodeMode {
    pub fn from_string(encode_mode: &str) -> Result<EncodeMode, String> {
        match encode_mode {
            "min" => Ok(EncodeMode::Min),
            "max" => Ok(EncodeMode::Max),
            "smart" => Ok(EncodeMode::Smart),
            _ => Err(format!("Unknown encode mode: {}", encode_mode)),
        }
    }
}

pub enum DisplayMode {
    Pretty,
    Accessible,
    TiAscii,
}

impl DisplayMode {
    pub fn from_string(display_mode: &str) -> Result<DisplayMode, String> {
        match display_mode {
            "pretty" => Ok(DisplayMode::Pretty),
            "accessible" => Ok(DisplayMode::Accessible),
            "ti" => Ok(DisplayMode::TiAscii),
            _ => Err(format!("Unknown display mode: {}", display_mode)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DisplayMode::Pretty => "pretty",
            DisplayMode::Accessible => "accessible",
            DisplayMode::TiAscii => "ti",
        }
        .to_string()
    }
}

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
            Metadata and Body Length: {}",
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
    fn from_byte(byte: u8) -> Result<FileType, String> {
        match byte {
            0x05 => Ok(FileType::Program),
            0x06 => Ok(FileType::LockedProgram),
            0x17 => Ok(FileType::Group),
            0x24 => Ok(FileType::FlashApplication),
            _ => Err(format!("Unknown file type byte: {:02X?}", byte)),
        }
    }

    fn to_byte(&self) -> u8 {
        match self {
            FileType::Program => 0x05,
            FileType::LockedProgram => 0x06,
            FileType::Group => 0x17,
            FileType::FlashApplication => 0x24,
        }
    }

    fn from_string(file_type: &str) -> Result<FileType, String> {
        match file_type {
            "Program" => Ok(FileType::Program),
            "Locked Program" => Ok(FileType::LockedProgram),
            "Group" => Ok(FileType::Group),
            "Flash Application" => Ok(FileType::FlashApplication),
            _ => Err(format!("Unknown file type string: {}", file_type)),
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
    fn from_byte(byte: u8) -> Result<Archived, String> {
        match byte {
            0x00 => Ok(Archived::NotArchived),
            0x80 => Ok(Archived::Archived),
            _ => Err(format!("Unknown archived byte: {:02X?}", byte)),
        }
    }

    fn to_byte(&self) -> u8 {
        match self {
            Archived::NotArchived => 0x00,
            Archived::Archived => 0x80,
        }
    }

    fn from_string(archived: &str) -> Result<Archived, String> {
        match archived {
            "Not Archived" => Ok(Archived::NotArchived),
            "Archived" => Ok(Archived::Archived),
            _ => Err(format!("Unknown archived string: {}", archived)),
        }
    }

    fn to_string(&self) -> String {
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

#[derive(Debug, Eq)]
pub enum Model {
    TI82,
    TI83,
    TI82ST,
    TI82STFR,
    TI76FR,
    TI83Plus,
    TI83PlusSE,
    TI83PlusFR,
    TI82Plus,
    TI84Plus,
    TI84PlusSE,
    TI83PlusFRUSB,
    TI84PFR,
    TI84PlusPSE,
    TI82A,
    TI84PlusT,
    TI84PlusCSE,
    TI84PlusCE,
    TI84PlusCET,
    TI83PCE,
    TI83PCEEP,
    TI84PlusCEPY,
    TI84PlusCETPE,
    TI82AEP,
    Latest,
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Model {
    pub fn from_string(model: &str) -> Result<Model, String> {
        match model {
            "TI-82" => Ok(Model::TI82),
            "TI-83" => Ok(Model::TI83),
            "TI-82ST" => Ok(Model::TI82ST),
            "TI-82ST.fr" => Ok(Model::TI82STFR),
            "TI-76.fr" => Ok(Model::TI76FR),
            "TI-83+" => Ok(Model::TI83Plus),
            "TI-83+SE" => Ok(Model::TI83PlusSE),
            "TI-83+.fr" => Ok(Model::TI83PlusFR),
            "TI-82+" => Ok(Model::TI82Plus),
            "TI-84+" => Ok(Model::TI84Plus),
            "TI-84+SE" => Ok(Model::TI84PlusSE),
            "TI-83+.fr:USB" => Ok(Model::TI83PlusFRUSB),
            "TI-84P.fr" => Ok(Model::TI84PFR),
            "TI-84+PSE" => Ok(Model::TI84PlusPSE),
            "TI-82A" => Ok(Model::TI82A),
            "TI-84+T" => Ok(Model::TI84PlusT),
            "TI-84+CSE" => Ok(Model::TI84PlusCSE),
            "TI-84+CE" => Ok(Model::TI84PlusCE),
            "TI-84+CET" => Ok(Model::TI84PlusCET),
            "TI-83PCE" => Ok(Model::TI83PCE),
            "TI-83PCEEP" => Ok(Model::TI83PCEEP),
            "TI-84+CEPY" => Ok(Model::TI84PlusCEPY),
            "TI-84+CETPE" => Ok(Model::TI84PlusCETPE),
            "TI-82AEP" => Ok(Model::TI82AEP),
            "latest" => Ok(Model::Latest),
            _ => Err(format!("Unknown model: {}", model)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Model::TI82 => "TI-82",
            Model::TI83 => "TI-83",
            Model::TI82ST => "TI-82ST",
            Model::TI82STFR => "TI-82ST.fr",
            Model::TI76FR => "TI-76.fr",
            Model::TI83Plus => "TI-83+",
            Model::TI83PlusSE => "TI-83+SE",
            Model::TI83PlusFR => "TI-83+.fr",
            Model::TI82Plus => "TI-82+",
            Model::TI84Plus => "TI-84+",
            Model::TI84PlusSE => "TI-84+SE",
            Model::TI83PlusFRUSB => "TI-83+.fr:USB",
            Model::TI84PFR => "TI-84P.fr",
            Model::TI84PlusPSE => "TI-84+PSE",
            Model::TI82A => "TI-82A",
            Model::TI84PlusT => "TI-84+T",
            Model::TI84PlusCSE => "TI-84+CSE",
            Model::TI84PlusCE => "TI-84+CE",
            Model::TI84PlusCET => "TI-84+CET",
            Model::TI83PCE => "TI-83PCE",
            Model::TI83PCEEP => "TI-83PCEEP",
            Model::TI84PlusCEPY => "TI-84+CEPY",
            Model::TI84PlusCETPE => "TI-84+CETPE",
            Model::TI82AEP => "TI-82AEP",
            Model::Latest => "latest",
        }
        .to_string()
    }

    pub fn model_order(&self) -> u32 {
        match self {
            Model::TI82 => 10,
            Model::TI83 | Model::TI82ST | Model::TI82STFR | Model::TI76FR => 20,
            Model::TI83Plus | Model::TI83PlusSE | Model::TI83PlusFR | Model::TI82Plus => 30,
            Model::TI84Plus
            | Model::TI84PlusSE
            | Model::TI83PlusFRUSB
            | Model::TI84PFR
            | Model::TI84PlusPSE => 40,
            Model::TI82A | Model::TI84PlusT => 45,
            Model::TI84PlusCSE => 50,
            Model::TI84PlusCE
            | Model::TI84PlusCET
            | Model::TI83PCE
            | Model::TI83PCEEP
            | Model::TI84PlusCEPY
            | Model::TI84PlusCETPE
            | Model::TI82AEP => 60,
            Model::Latest => 9999999,
        }
    }

    pub fn display_models() {
        println!(
            "TI-82\n\
            TI-83\n\
            TI-82ST\n\
            TI-82ST.fr\n\
            TI-76.fr\n\
            TI-83+\n\
            TI-83+SE\n\
            TI-83+.fr\n\
            TI-82+\n\
            TI-84+\n\
            TI-84+SE\n\
            TI-83+.fr:USB\n\
            TI-84P.fr\n\
            TI-84+PSE\n\
            TI-82A\n\
            TI-84+T\n\
            TI-84+CSE\n\
            TI-84+CE\n\
            TI-84+CET\n\
            TI-83PCE\n\
            TI-83PCEEP\n\
            TI-84+CEPY\n\
            TI-84+CETPE\n\
            TI-82AEP\n\
            latest"
        );
    }
}
