use crate::calculator::program::{Archived, Program};
use crate::calculator::{DisplayMode, Model};
use crate::tokens::OsVersion;
use std::path::Path;

#[test]
fn test_archive_and_unarchive() {
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

    let result = program.metadata.archive();

    assert!(result.is_ok(), "Failed to lock the program: {:?}", result);

    assert!(
        program.metadata.bytes[15] == Archived::Archived.to_byte(),
        "The archived byte in the metadata is not correct: {:02X?}",
        program.metadata.bytes
    );

    assert!(
        program.metadata.archived.to_byte() == Archived::Archived.to_byte(),
        "The archived status in the metadata is not correct: {:02X?}",
        program.metadata.archived.to_string()
    );

    let result = program.metadata.unarchive();

    assert!(
        result.is_ok(),
        "Failed to unlock the program: {:02X?}",
        result
    );

    assert!(
        program.metadata.bytes[15] == Archived::NotArchived.to_byte(),
        "The un-archived byte in the metadata is not correct: {:02X?}",
        program.metadata.bytes
    );

    assert!(
        program.metadata.archived.to_byte() == Archived::NotArchived.to_byte(),
        "The un-archived status in the metadata is not correct: {:02X?}",
        program.metadata.archived.to_string()
    );

    assert!(
        program.metadata.bytes.len() == 19,
        "The metadata is not the correct length: {:02X?}",
        program.metadata.bytes
    );
}
