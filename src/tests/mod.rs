use crate::program::{DisplayMode, EncodeMode, Program};
use crate::tokens::OsVersion;
use std::path::Path;

#[test]
fn test_decode_and_encode() {
    test_program(
        "./src/tests/programs/BASECONV.8xp",
        "./src/tests/programs/BASECONV.txt",
    );
    test_program(
        "./src/tests/programs/DBD.8xp",
        "./src/tests/programs/DBD.txt",
    );
    test_program(
        "./src/tests/programs/FACTOR.8xp",
        "./src/tests/programs/FACTOR.txt",
    );
    test_program(
        "./src/tests/programs/MATHTOOL.8xp",
        "./src/tests/programs/MATHTOOL.txt",
    );
    test_program(
        "./src/tests/programs/POLCONIC.8xp",
        "./src/tests/programs/POLCONIC.txt",
    );
    test_program(
        "./src/tests/programs/QUEUESIM.8xp",
        "./src/tests/programs/QUEUESIM.txt",
    );
    test_program(
        "./src/tests/programs/RADICAL.8xp",
        "./src/tests/programs/RADICAL.txt",
    );
    test_program(
        "./src/tests/programs/SPLINE.8xp",
        "./src/tests/programs/SPLINE.txt",
    );
    test_program(
        "./src/tests/programs/TOCCATA.8xp",
        "./src/tests/programs/TOCCATA.txt",
    );
}

fn test_program(path_to_8xp: &str, path_to_txt: &str) {
    let path_to_8xp = Path::new(&path_to_8xp);
    let path_to_txt = Path::new(&path_to_txt);

    let loaded_8xp = Program::load_from_8xp(
        path_to_8xp.to_path_buf(),
        OsVersion {
            model: "latest".to_string(),
            version: "latest".to_string(),
        },
        DisplayMode::Accessible,
    );

    assert!(
        loaded_8xp.is_ok(),
        "Failed to load 8xp file: {:?}",
        path_to_8xp
    );

    let loaded_txt = Program::load_from_txt(
        path_to_txt.to_path_buf(),
        OsVersion {
            model: "latest".to_string(),
            version: "latest".to_string(),
        },
        EncodeMode::Smart,
    );

    assert!(
        loaded_txt.is_ok(),
        "Failed to load txt file: {:?}",
        path_to_txt
    );

    let loaded_8xp = loaded_8xp.unwrap();
    let loaded_txt = loaded_txt.unwrap();

    // checks within files
    assert_eq!(
        loaded_8xp.header.bytes.len(),
        55,
        "The header of the program is not 55 bytes {:?}",
        path_to_8xp,
    );

    assert_eq!(
        loaded_8xp.metadata.bytes.len(),
        19,
        "The checksum of the program is not 19 bytes {:?}",
        path_to_8xp,
    );

    assert_eq!(
        loaded_8xp.checksum.bytes.len(),
        2,
        "The checksum of the program is not 2 bytes {:?}",
        path_to_8xp,
    );

    assert_eq!(
        loaded_txt.header.bytes.len(),
        55,
        "The header of the program is not 55 bytes {:?}",
        path_to_txt,
    );

    assert_eq!(
        loaded_txt.metadata.bytes.len(),
        19,
        "The checksum of the program is not 19 bytes {:?}",
        path_to_txt,
    );

    assert_eq!(
        loaded_txt.checksum.bytes.len(),
        2,
        "The checksum of the program is not 2 bytes {:?}",
        path_to_txt,
    );

    assert_eq!(
        loaded_8xp.metadata.body_and_checksum_length,
        loaded_8xp.metadata.body_and_checksum_length_copy,
        "The body and checksum length and body and checksum length copy of the program do not match {:?}",
        path_to_8xp,
    );

    assert_eq!(
        loaded_txt.metadata.body_and_checksum_length,
        loaded_txt.metadata.body_and_checksum_length_copy,
        "The body and checksum length and body and checksum length copy of the program do not match {:?}",
        path_to_txt,
    );

    assert_eq!(
        loaded_8xp.metadata.body_and_checksum_length,
        loaded_8xp.metadata.body_length + 2,
        "The body and checksum length of the program is not the body length plus 2 {:?}",
        path_to_8xp,
    );

    assert_eq!(
        loaded_txt.metadata.body_and_checksum_length,
        loaded_txt.metadata.body_length + 2,
        "The body and checksum length of the program is not the body length plus 2 {:?}",
        path_to_txt,
    );

    // checks between files
    assert_eq!(
        loaded_8xp.body.bytes, loaded_txt.body.bytes,
        "The bytes of the programs do not match between {:?} and {:?}",
        path_to_8xp, path_to_txt
    );

    assert_eq!(
        loaded_8xp.body.translation, loaded_txt.body.translation,
        "The translation of the programs do not match between {:?} and {:?}",
        path_to_8xp, path_to_txt
    );

    assert_eq!(
        loaded_8xp.metadata.body_and_checksum_length, loaded_txt.metadata.body_and_checksum_length,
        "The body and checksum length of the programs do not match between {:?} and {:?}",
        path_to_8xp, path_to_txt
    );

    assert_eq!(
        loaded_8xp.metadata.body_and_checksum_length_copy,
        loaded_txt.metadata.body_and_checksum_length_copy,
        "The body and checksum length copy of the programs do not match between {:?} and {:?}",
        path_to_8xp,
        path_to_txt
    );

    assert_eq!(
        loaded_8xp.metadata.body_length, loaded_txt.metadata.body_length,
        "The body length of the programs do not match between {:?} and {:?}",
        path_to_8xp, path_to_txt
    );

    assert_eq!(
        loaded_8xp.header.metadata_and_body_length, loaded_txt.header.metadata_and_body_length,
        "The metadata and body length of the programs do not match between {:?} and {:?}",
        path_to_8xp, path_to_txt
    );

    assert_eq!(
        loaded_8xp.checksum.bytes, loaded_txt.checksum.bytes,
        "The checksum bytes of the programs do not match between {:?} and {:?}",
        path_to_8xp, path_to_txt
    );

    assert_eq!(
        loaded_8xp.checksum.value, loaded_txt.checksum.value,
        "The checksum value of the programs do not match between {:?} and {:?}",
        path_to_8xp, path_to_txt
    );
}

#[test]
fn test_rename() {
    let input_path = Path::new("./src/tests/programs/RADICAL.8xp");

    let target_version = OsVersion {
        model: "latest".to_string(),
        version: "latest".to_string(),
    };

    let program = Program::load_from_8xp(
        input_path.to_path_buf(),
        target_version,
        DisplayMode::Accessible,
    );

    assert!(program.is_ok(), "Failed to load program: {:?}", input_path);

    let mut program = program.unwrap();

    let before_bytes = program.metadata.bytes.clone();

    let result = program.metadata.rename("TESTINGS".to_string());

    assert!(result.is_ok(), "Failed to rename program: {:?}", result);

    let after_bytes = &program.metadata.bytes;

    //before: RADICAL[0D, 00, 74, 02, 05, 52, 41, 44, 49, 43, 41, 4C, 00, 00, 00, 74, 02, 72, 02]
    //after: TESTINGS[0D, 00, 74, 02, 05, 74, 65, 73, 74, 00, 00, 00, 00, 00, 00, 74, 02, 72, 02]
    assert_ne!(
        before_bytes[5..13],
        after_bytes[5..13],
        "The name of the program did not change: (before: {:02X?}, after: {:02X?})",
        before_bytes,
        after_bytes,
    );

    assert_eq!(
        before_bytes[0..5],
        after_bytes[0..5],
        "The first part of the metadata was altered: (before: {:02X?}, after: {:02X?})",
        before_bytes,
        after_bytes
    );

    assert_eq!(
        before_bytes[13..19],
        after_bytes[13..19],
        "The second part of the metadata was altered: (before: {:02X?}, after: {:02X?})",
        before_bytes,
        after_bytes
    );

    assert!(
        after_bytes.len() == 19,
        "The metadata is not the correct length: {:?}",
        after_bytes
    );
}

#[test]
fn test_rename_fail_on_length() {
    let input_path = Path::new("./src/tests/programs/RADICAL.8xp");

    let target_version = OsVersion {
        model: "latest".to_string(),
        version: "latest".to_string(),
    };

    let program = Program::load_from_8xp(
        input_path.to_path_buf(),
        target_version,
        DisplayMode::Accessible,
    );

    assert!(program.is_ok(), "Failed to load program: {:?}", input_path);

    let mut program = program.unwrap();

    let result = program.metadata.rename("THISNAMEISTOLONG".to_string());

    assert!(
        !result.is_ok(),
        "Failed to catch a name that is too long: {:?}",
        result
    );

    assert!(
        program.metadata.bytes.len() == 19,
        "The metadata is not the correct length: {:?}",
        program.metadata.bytes
    );
}

#[test]
fn test_rename_fail_on_characters() {
    let input_path = Path::new("./src/tests/programs/RADICAL.8xp");

    let target_version = OsVersion {
        model: "latest".to_string(),
        version: "latest".to_string(),
    };

    let program = Program::load_from_8xp(
        input_path.to_path_buf(),
        target_version,
        DisplayMode::Accessible,
    );

    assert!(program.is_ok(), "Failed to load program: {:?}", input_path);

    let mut program = program.unwrap();

    let result = program.metadata.rename("1823".to_string());

    assert!(
        !result.is_ok(),
        "Failed to detect numbers in name: {:?}",
        result
    );

    let result = program.metadata.rename("    ".to_string());

    assert!(
        !result.is_ok(),
        "Failed to detect spaces in name: {:?}",
        result
    );

    let result = program.metadata.rename("!@#$%^&*".to_string());

    assert!(
        !result.is_ok(),
        "Failed to detect weird characters in name: {:?}",
        result
    );

    assert!(
        program.metadata.bytes.len() == 19,
        "The metadata is not the correct length: {:?}",
        program.metadata.bytes
    );
}
