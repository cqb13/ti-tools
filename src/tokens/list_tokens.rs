use std::collections::HashMap;

pub fn get_list_tokens() -> HashMap<u8, String> {
    let tokens: HashMap<u8, String> = [
        (0x00, "L1".to_string()),
        (0x01, "L2".to_string()),
        (0x02, "L3".to_string()),
        (0x03, "L4".to_string()),
        (0x04, "L5".to_string()),
        (0x05, "L6".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    tokens
}
