use crate::calculator::decode::decode;
use crate::calculator::models::ModelDetails;
use crate::calculator::program::{
    Body, Checksum, Destination, FileType, Header, Metadata, ProgramFileType,
};
use crate::calculator::DisplayMode;
use crate::errors::CliError;
use crate::tokens::{load_tokens, OsVersion};
use std::path::PathBuf;

pub fn create_from_8xp(
    path: PathBuf,
    file_type: ProgramFileType,
    display_mode: &DisplayMode,
) -> Result<(Header, Metadata, Body, Checksum, ModelDetails), CliError> {
    let bytes = match std::fs::read(&path).map_err(|err| err.to_string()) {
        Ok(bytes) => bytes,
        Err(err) => return Err(CliError::FileRead(err)),
    };

    let (header_bytes, bytes) = bytes.split_at(55);
    let (metadata_bytes, bytes) = match file_type {
        ProgramFileType::XP => bytes.split_at(19),
        ProgramFileType::XPThree | ProgramFileType::XPTwo => bytes.split_at(17),
        ProgramFileType::TXT => {
            return Err(CliError::FileRead(
                "TXT files cant be loaded as 8xp files".to_string(),
            ))
        }
        ProgramFileType::JSON => {
            return Err(CliError::FileRead(
                "JSON files cant be loaded as 8xp files".to_string(),
            ))
        }
    };
    let (body_bytes, checksum_bytes) = bytes.split_at(bytes.len() - 2);

    // header translation
    let signature = header_bytes[0..8]
        .iter()
        .map(|byte| *byte as char)
        .collect::<String>();

    let signature_extra = [header_bytes[8], header_bytes[9]];

    let product_id = header_bytes[10];

    let model_details = ModelDetails::from_byte(product_id, &signature)?;

    let version = OsVersion {
        model: model_details.model.clone(),
        version: "latest".to_string(),
    };

    let tokens = load_tokens(&version)?;

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
    let metadata = match file_type {
        ProgramFileType::XP => {
            let flag = metadata_bytes[0];
            let unknown = metadata_bytes[1];
            let body_and_checksum_length = [metadata_bytes[2], metadata_bytes[3]];
            let file_type = FileType::from_byte(metadata_bytes[4])?;
            let name = metadata_bytes[5..13]
                .iter()
                .filter(|byte| **byte != 0x00)
                .map(|byte| *byte as char)
                .collect::<String>();
            let version = metadata_bytes[13];
            let destination = Destination::from_byte(metadata_bytes[14])?;
            let body_and_checksum_length_duplicate = [metadata_bytes[15], metadata_bytes[16]];
            let body_length = [metadata_bytes[17], metadata_bytes[18]];

            Metadata::new(
                metadata_bytes.to_vec(),
                flag,
                unknown,
                u16::from_le_bytes(body_and_checksum_length),
                file_type,
                name,
                version,
                destination,
                u16::from_le_bytes(body_and_checksum_length_duplicate),
                u16::from_le_bytes(body_length),
            )
        }
        ProgramFileType::XPThree | ProgramFileType::XPTwo => {
            let mut new_metadata_bytes = Vec::new();

            new_metadata_bytes.push(0x0D);
            new_metadata_bytes.push(0x00);
            let body_and_checksum_length = (body_bytes.len() + 2) as u16;
            new_metadata_bytes.extend(body_and_checksum_length.to_le_bytes());
            new_metadata_bytes.push(0x05);
            let name = metadata_bytes[5..13]
                .iter()
                .filter(|byte| **byte != 0x00)
                .map(|byte| *byte as char)
                .collect::<String>();
            new_metadata_bytes.extend(name.as_bytes());
            new_metadata_bytes.push(0x00);
            new_metadata_bytes.extend(body_and_checksum_length.to_le_bytes());
            let body_length = (body_bytes.len() + 2) as u16;
            new_metadata_bytes.extend(body_length.to_le_bytes());

            Metadata::new(
                new_metadata_bytes,
                0x0D,
                0x00,
                body_and_checksum_length,
                FileType::Program,
                name,
                0x00,
                Destination::RAM,
                body_and_checksum_length,
                body_length,
            )
        }
        ProgramFileType::TXT => {
            return Err(CliError::FileRead(
                "TXT files cant be loaded as 8xp files".to_string(),
            ))
        }
        ProgramFileType::JSON => {
            return Err(CliError::FileRead(
                "JSON files cant be loaded as 8xp files".to_string(),
            ))
        }
    };

    // body translation
    let translation = decode(body_bytes, &tokens, "en", &display_mode)?;

    let body = Body::new(body_bytes.to_vec(), translation);

    // checksum translation
    let checksum_value = u16::from_le_bytes([checksum_bytes[0], checksum_bytes[1]]);

    let checksum = Checksum::new(checksum_bytes.to_vec(), checksum_value);

    Ok((header, metadata, body, checksum, model_details))
}
