use crate::calculator::program::Program;
use crate::calculator::{DisplayMode, Model};
use crate::tokens::OsVersion;
use std::path::Path;

#[test]
fn test_rename() {
    let input_path = Path::new("./src/tests/programs/RADICAL.8xp");

    let target_version = OsVersion {
        model: Model::Latest,
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
        model: Model::Latest,
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
        model: Model::Latest,
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
