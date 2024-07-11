use crate::calculator::program::{FileType, Program};
use crate::calculator::{DisplayMode, Model};
use crate::tokens::OsVersion;
use std::path::Path;

#[test]
fn test_lock_and_unlock() {
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

    let result = program.metadata.lock();

    assert!(result.is_ok(), "Failed to lock the program: {:?}", result);

    assert!(
        program.metadata.bytes[4] == FileType::LockedProgram.to_byte(),
        "The locked byte in the metadata is not correct: {:?}",
        program.metadata.bytes
    );

    assert!(
        program.metadata.file_type.to_byte() == FileType::LockedProgram.to_byte(),
        "The file_type in the metadata is not correct: {:?}",
        program.metadata.file_type.to_string()
    );

    let result = program.metadata.unlock();

    assert!(result.is_ok(), "Failed to unlock the program: {:?}", result);

    assert!(
        program.metadata.bytes[4] == FileType::Program.to_byte(),
        "The unlocked byte in the metadata is not correct: {:?}",
        program.metadata.bytes
    );

    assert!(
        program.metadata.file_type.to_byte() == FileType::Program.to_byte(),
        "The file_type in the metadata is not correct: {:?}",
        program.metadata.file_type.to_string()
    );

    assert!(
        program.metadata.bytes.len() == 19,
        "The metadata is not the correct length: {:?}",
        program.metadata.bytes
    );
}
