use std::path::Path;
use ti_tools::calculator::program::{FileType, Program};
use ti_tools::calculator::DisplayMode;

#[test]
fn test_lock_and_unlock() {
    let input_path = Path::new("./tests/programs/RADICAL.8xp");

    let program = Program::load_from_8xp(input_path.to_path_buf(), DisplayMode::Accessible);

    assert!(program.is_ok(), "Failed to load program: {:?}", input_path);

    let mut program = program.unwrap();

    program.metadata.lock();

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

    program.metadata.unlock();

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
