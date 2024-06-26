use std::collections::HashMap;

pub fn get_picture_tokens() -> HashMap<u8, String> {
    let tokens: HashMap<u8, String> = [
        (0x00, "Pic1".to_string()),
        (0x01, "Pic2".to_string()),
        (0x02, "Pic3".to_string()),
        (0x03, "Pic4".to_string()),
        (0x04, "Pic5".to_string()),
        (0x05, "Pic6".to_string()),
        (0x06, "Pic7".to_string()),
        (0x07, "Pic8".to_string()),
        (0x08, "Pic9".to_string()),
        (0x09, "Pic0".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    tokens
}
