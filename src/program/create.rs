use super::decode::decode;
use super::encode::encode;
use super::{Archived, Body, Checksum, DisplayMode, FileType, Header, Metadata};
use crate::tokens::{load_tokens, OsVersion};
use std::path::PathBuf;

pub fn create_from_txt(
    path: PathBuf,
    version: &OsVersion,
) -> Result<(Header, Metadata, Body, Checksum), String> {
    let tokens = load_tokens(version);

    let file_string = std::fs::read_to_string(&path).map_err(|err| err.to_string())?;

    let name = file_string
        .lines()
        .next()
        .expect("missing file name")
        .to_string();
    let file_type = FileType::from_string(file_string.lines().nth(1).expect("missing file type"));
    let archived = Archived::from_string(file_string.lines().nth(2).expect("missing archived"));
    let display_mode =
        DisplayMode::from_string(file_string.lines().nth(3).expect("missing display mode"));

    if name.len() > 8 {
        return Err("name must be 8 or less characters".to_string());
    }

    if !name.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("name must be alphabetic".to_string());
    }

    let body_string = file_string
        .lines()
        .skip(4)
        .collect::<Vec<&str>>()
        .join("\n");

    let body_bytes = encode(&body_string, &tokens, true, display_mode);

    let mut header_bytes = Vec::new();
    let signature = "**TI83F*";
    header_bytes.extend(signature.as_bytes());
    header_bytes.extend([0x1A, 0x0A]); // signature part 2
    header_bytes.push(0x00); // mystery b
    let mut comment = create_comment().as_bytes().to_owned();
    while comment.len() != 42 {
        comment.push(0x00)
    }
    header_bytes.extend(comment);
    let meta_and_body_length = (body_bytes.len() + 19) as u16;
    header_bytes.extend(meta_and_body_length.to_le_bytes());

    let header = Header::new(
        header_bytes,
        signature.to_string(),
        vec![0x1A, 0x0A],
        0x00,
        create_comment(),
        meta_and_body_length,
    );

    let mut metadata_bytes = Vec::new();
    metadata_bytes.push(0x0D); // flag
    metadata_bytes.push(0x00); // unknown byte
    let body_and_checksum_length = (body_bytes.len() + 2) as u16;
    metadata_bytes.extend(body_and_checksum_length.to_le_bytes());
    metadata_bytes.push(file_type.to_byte());
    let mut name_bytes = name.as_bytes().to_vec();
    while name_bytes.len() != 8 {
        name_bytes.push(0x00)
    }
    metadata_bytes.extend(name_bytes);
    metadata_bytes.push(0x00); // version
    metadata_bytes.push(archived.to_byte());
    metadata_bytes.extend(body_and_checksum_length.to_le_bytes());
    let body_length = body_bytes.len() as u16;
    metadata_bytes.extend(body_length.to_le_bytes());

    let metadata = Metadata::new(
        metadata_bytes,
        0x0D,
        0x00,
        body_and_checksum_length,
        file_type,
        name,
        0x00,
        archived,
        body_and_checksum_length,
        body_length,
    );

    let checksum_bytes = (body_bytes.len() as u16).to_le_bytes().to_vec();

    if checksum_bytes.len() != 2 {
        return Err("checksum length is not 2".to_string());
    }

    let checksum = Checksum::new(checksum_bytes, body_bytes.len() as u16);

    let body = Body::new(body_bytes, body_string);

    Ok((header, metadata, body, checksum))
}

fn create_comment() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("Created by TI-Tools {}", version)
}

pub fn create_from_8xp(
    path: PathBuf,
    version: &OsVersion,
    display_mode: &DisplayMode,
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
    let translation = decode(body_bytes, &tokens, "en", &display_mode)?;

    let body = Body::new(body_bytes.to_vec(), translation);

    // checksum translation
    let value = u16::from_le_bytes([checksum[0], checksum[1]]);

    let checksum = Checksum::new(checksum.to_vec(), value);

    Ok((header, metadata, body, checksum))
}
