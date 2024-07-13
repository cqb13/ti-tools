use crate::calculator::models::Model;
use crate::calculator::program::Program;
use crate::calculator::DisplayMode;
use crate::tokens::OsVersion;
use std::path::Path;

#[test]
fn test_comment() {
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

    let before_bytes = program.header.bytes.clone();

    let result = program.header.comment("This is a new comment".to_string());

    assert!(result.is_ok(), "Failed to comment on program: {:?}", result);

    let after_bytes = &program.header.bytes;

    assert_ne!(
        before_bytes[11..53],
        after_bytes[11..53],
        "The comment of the program did not change: (before: {:02X?}, after: {:02X?})",
        before_bytes,
        after_bytes,
    );

    assert_eq!(
        before_bytes[0..11],
        after_bytes[0..11],
        "The first part of the header was altered: (before: {:02X?}, after: {:02X?})",
        before_bytes,
        after_bytes
    );

    assert_eq!(
        before_bytes[53..55],
        after_bytes[53..55],
        "The second part of the header was altered: (before: {:02X?}, after: {:02X?})",
        before_bytes,
        after_bytes
    );

    assert!(
        after_bytes.len() == 55,
        "The header is not the correct length: {:?}",
        after_bytes
    );
}

#[test]
fn test_comment_fail_on_length() {
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

    let result = program.header.comment(
        "123456789101112131415161718192021222324252627282930313233343536373839404142".to_string(),
    );

    assert!(
        !result.is_ok(),
        "Failed to catch a comment that is too long: {:?}",
        result
    );

    assert!(
        program.header.bytes.len() == 55,
        "The header is not the correct length: {:?}",
        program.header.bytes
    );
}
