use std::collections::HashMap;

pub fn get_gdb_tokens() -> HashMap<u8, String> {
    let tokens: HashMap<u8, String> = [
        (0x00, "GDB1".to_string()),
        (0x01, "GDB2".to_string()),
        (0x02, "GDB3".to_string()),
        (0x03, "GDB4".to_string()),
        (0x04, "GDB5".to_string()),
        (0x05, "GDB6".to_string()),
        (0x06, "GDB7".to_string()),
        (0x07, "GDB8".to_string()),
        (0x08, "GDB9".to_string()),
        (0x09, "GDB0".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    tokens
}
