use super::decode::decode;
use super::{Archived, Body, Checksum, DisplayMode, FileType, Header, Metadata};
use crate::tokens::{load_tokens, OsVersion};
use std::path::PathBuf;

pub fn create_from_8xp(
    path: PathBuf,
    version: &OsVersion,
    display_mode: DisplayMode,
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
    let translation = decode(body_bytes, &tokens, "en", display_mode)?;

    let body = Body::new(body_bytes.to_vec(), translation);

    // checksum translation
    let value = u16::from_le_bytes([checksum[0], checksum[1]]);

    let checksum = Checksum::new(checksum.to_vec(), value);

    Ok((header, metadata, body, checksum))
}
