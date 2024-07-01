use std::path::PathBuf;

mod decode;

use crate::tokens::{load_tokens, OsVersion};
use decode::decode;

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
        let file_type = match get_file_type(&path) {
            Ok(file_type) => file_type,
            Err(err) => return Err(err),
        };

        let (header, metadata, body, checksum) = match file_type {
            ProgramFileType::XP => load_from_8xp(&version, path),
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

    pub fn display(&self) -> String {
        let header = self.header.display();
        let metadata = self.metadata.display();
        let body = self.body.display();
        let checksum = self.checksum.display();

        format!("{}\n{}\n{}\n{}", header, metadata, body, checksum)
    }
}

pub struct Header {
    bytes: Vec<u8>,
    signature: String,
    signature_extra: Vec<u8>,
    product_id: u8,
    comment: String,
    metadata_and_body_length: u16,
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
    bytes: Vec<u8>,
    translation: String,
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
    bytes: Vec<u8>,
    value: u16,
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

fn load_from_8xp(
    version: &OsVersion,
    path: PathBuf,
) -> Result<(Header, Metadata, Body, Checksum), String> {
    let tokens = load_tokens(version);

    let bytes = std::fs::read(&path).map_err(|err| err.to_string())?;

    let (header_bytes, bytes) = bytes.split_at(55);
    let (metadata_bytes, bytes) = bytes.split_at(19);
    let (body_bytes, checksum) = bytes.split_at(bytes.len() - 2);

    // header translation
    let signature = header_bytes[0..8]
        .iter()
        .map(|byte| *byte as char)
        .collect::<String>();

    let signature_extra = [header_bytes[8], header_bytes[9]];

    let product_id = header_bytes[10];

    let comment = header_bytes[11..53]
        .iter()
        .filter(|byte| **byte != 0x00)
        .map(|byte| *byte as char)
        .collect::<String>();

    let metadata_and_body_length = [header_bytes[53], header_bytes[54]];

    let header = Header::new(
        header_bytes.to_vec(),
        signature,
        signature_extra.to_vec(),
        product_id,
        comment,
        u16::from_le_bytes(metadata_and_body_length),
    );

    // metadata translation
    let flag = metadata_bytes[0];
    let unknown = metadata_bytes[1];
    let body_and_checksum_length = [metadata_bytes[2], metadata_bytes[3]];
    let file_type = FileType::from_byte(metadata_bytes[4]);
    let name = metadata_bytes[5..13]
        .iter()
        .filter(|byte| **byte != 0x00)
        .map(|byte| *byte as char)
        .collect::<String>();
    let version = metadata_bytes[13];
    let archived = Archived::from_byte(metadata_bytes[14]);
    let body_and_checksum_length_duplicate = [metadata_bytes[15], metadata_bytes[16]];
    let body_length = [metadata_bytes[17], metadata_bytes[18]];

    let metadata = Metadata::new(
        metadata_bytes.to_vec(),
        flag,
        unknown,
        u16::from_le_bytes(body_and_checksum_length),
        file_type,
        name,
        version,
        archived,
        u16::from_le_bytes(body_and_checksum_length_duplicate),
        u16::from_le_bytes(body_length),
    );

    // body translation
    let translation = decode(body_bytes, &tokens, "en", DisplayMode::Accessible)?;

    let body = Body::new(body_bytes.to_vec(), translation);

    // checksum translation
    let value = u16::from_le_bytes([checksum[0], checksum[1]]);

    let checksum = Checksum::new(checksum.to_vec(), value);

    Ok((header, metadata, body, checksum))
}

enum ProgramFileType {
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

enum FileType {
    Program,
    EditLockedProgram,
    Group,
    FlashApplication,
}

impl FileType {
    fn from_byte(byte: u8) -> FileType {
        match byte {
            0x05 => FileType::Program,
            0x06 => FileType::EditLockedProgram,
            0x17 => FileType::Group,
            0x24 => FileType::FlashApplication,
            _ => panic!("Unknown file type: {:02X?}", byte),
        }
    }

    fn to_string(&self) -> String {
        match self {
            FileType::Program => "Program".to_string(),
            FileType::EditLockedProgram => "Edit Locked Program".to_string(),
            FileType::Group => "Group".to_string(),
            FileType::FlashApplication => "Flash Application".to_string(),
        }
    }
}

enum Archived {
    Unarchived,
    Archived,
}

impl Archived {
    fn from_byte(byte: u8) -> Archived {
        match byte {
            0x00 => Archived::Unarchived,
            0x80 => Archived::Archived,
            _ => panic!("Unknown archived byte: {:02X?}", byte),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Archived::Unarchived => "Unarchived".to_string(),
            Archived::Archived => "Archived".to_string(),
        }
    }
}
