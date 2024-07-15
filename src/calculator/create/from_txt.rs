use crate::calculator::encode::encode;
use crate::calculator::models::{Model, ModelDetails};
use crate::calculator::program::{Archived, Body, Checksum, FileType, Header, Metadata};
use crate::calculator::{DisplayMode, EncodeMode};
use crate::tokens::{load_tokens, OsVersion};
use std::path::PathBuf;

pub fn create_from_txt(
    path: PathBuf,
    encode_mode: EncodeMode,
) -> Result<(Header, Metadata, Body, Checksum, ModelDetails), String> {
    let file_string = std::fs::read_to_string(&path).map_err(|err| err.to_string())?;

    let name = file_string
        .lines()
        .next()
        .ok_or_else(|| "missing name".to_string())?;
    let line = file_string
        .lines()
        .nth(2)
        .ok_or_else(|| "missing file type".to_string())?;
    let file_type = FileType::from_string(line)?;
    let line = file_string
        .lines()
        .nth(3)
        .ok_or_else(|| "missing archived".to_string())?;
    let archived = Archived::from_string(line)?;
    let line = file_string
        .lines()
        .nth(4)
        .ok_or_else(|| "missing display mode".to_string())?;
    let display_mode = DisplayMode::from_string(line)?;
    let line = file_string
        .lines()
        .nth(5)
        .ok_or_else(|| "missing model".to_string())?;

    let model = Model::from_string(line);
    let model_details = ModelDetails::from_model(&model);

    let version = OsVersion {
        model,
        version: "latest".to_string(),
    };

    let tokens = load_tokens(&version)?;

    if name.len() > 8 {
        return Err("name must be 8 or less characters".to_string());
    }

    if !name.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("name must be alphabetic".to_string());
    }

    let body_string = file_string
        .lines()
        .skip(6)
        .collect::<Vec<&str>>()
        .join("\n");

    let body_bytes = encode(&body_string, &tokens, true, display_mode, encode_mode)?;

    let mut header_bytes = Vec::new();
    let signature = &model_details.signature.to_string();
    header_bytes.extend(signature.as_bytes());
    header_bytes.extend([0x1A, 0x0A]); // signature part 2
    header_bytes.push(model_details.product_id);
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
        model_details.product_id,
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

    let checksum: u32 = metadata_bytes.iter().map(|&byte| byte as u32).sum::<u32>()
        + body_bytes.iter().map(|&byte| byte as u32).sum::<u32>();
    let checksum = checksum.to_le_bytes()[0..2].to_vec();
    let checksum = [checksum[0], checksum[1]];

    let metadata = Metadata::new(
        metadata_bytes,
        0x0D,
        0x00,
        body_and_checksum_length,
        file_type,
        name.to_string(),
        0x00,
        archived,
        body_and_checksum_length,
        body_length,
    );

    let checksum = Checksum::new(checksum.to_vec(), u16::from_le_bytes(checksum));

    let body = Body::new(body_bytes, body_string);

    Ok((header, metadata, body, checksum, model_details))
}

fn create_comment() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("Created by TI-Tools {}", version)
}
