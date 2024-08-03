use std::path::Path;
use ti_tools::calculator::program::{Destination, Program};
use ti_tools::calculator::DisplayMode;

#[test]
fn test_archive_and_unarchive() {
    let input_path = Path::new("./tests/programs/RADICAL.8xp");

    let program = Program::load_from_8xp(input_path.to_path_buf(), DisplayMode::Accessible);

    assert!(program.is_ok(), "Failed to load program: {:?}", input_path);

    let mut program = program.unwrap();

    program.metadata.archive();

    assert!(
        program.metadata.bytes[14] == Destination::Archive.to_byte(),
        "The destination byte in the metadata is not correct: {:02X?}",
        program.metadata.bytes
    );

    assert!(
        program.metadata.destination.to_byte() == Destination::Archive.to_byte(),
        "The destination status in the metadata is not correct: {:02X?}",
        program.metadata.destination.to_string()
    );

    program.metadata.unarchive();

    assert!(
        program.metadata.bytes[14] == Destination::RAM.to_byte(),
        "The destination byte in the metadata is not correct: {:02X?}",
        program.metadata.bytes
    );

    assert!(
        program.metadata.destination.to_byte() == Destination::RAM.to_byte(),
        "The destination status in the metadata is not correct: {:02X?}",
        program.metadata.destination.to_string()
    );

    assert!(
        program.metadata.bytes.len() == 19,
        "The metadata is not the correct length: {:02X?}",
        program.metadata.bytes
    );
}
