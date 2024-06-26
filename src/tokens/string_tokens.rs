use std::collections::HashMap;

pub fn get_string_tokens() -> HashMap<u8, String> {
    let tokens: HashMap<u8, String> = [
        (0x00, "Str1".to_string()),
        (0x01, "Str2".to_string()),
        (0x02, "Str3".to_string()),
        (0x03, "Str4".to_string()),
        (0x04, "Str5".to_string()),
        (0x05, "Str6".to_string()),
        (0x06, "Str7".to_string()),
        (0x07, "Str8".to_string()),
        (0x08, "Str9".to_string()),
        (0x09, "Str0".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    tokens
}
