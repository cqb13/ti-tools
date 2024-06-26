use std::collections::HashMap;

pub fn get_matrix_tokens() -> HashMap<u8, String> {
    let tokens: HashMap<u8, String> = [
        (0x00, "[A]".to_string()),
        (0x01, "[B]".to_string()),
        (0x02, "[C]".to_string()),
        (0x03, "[D]".to_string()),
        (0x04, "[E]".to_string()),
        (0x05, "[F]".to_string()),
        (0x06, "[G]".to_string()),
        (0x07, "[H]".to_string()),
        (0x08, "[I]".to_string()),
        (0x09, "[J]".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    tokens
}
