use std::collections::HashMap;

pub fn get_list_tokens() -> HashMap<u8, String> {
    let tokens: HashMap<u8, String> = [
        (0x00, "L₁".to_string()),
        (0x01, "L₂".to_string()),
        (0x02, "L₃".to_string()),
        (0x03, "L₄".to_string()),
        (0x04, "L₅".to_string()),
        (0x05, "L₆".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    tokens
}
