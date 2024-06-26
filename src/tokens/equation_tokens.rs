use std::collections::HashMap;

pub fn get_equation_tokens() -> HashMap<u8, String> {
    let tokens: HashMap<u8, String> = [
        (0x10, "Y1".to_string()),
        (0x11, "Y2".to_string()),
        (0x12, "Y3".to_string()),
        (0x13, "Y4".to_string()),
        (0x14, "Y5".to_string()),
        (0x15, "Y6".to_string()),
        (0x16, "Y7".to_string()),
        (0x17, "Y8".to_string()),
        (0x18, "Y9".to_string()),
        (0x19, "Y0".to_string()),
        (0x20, "X1T".to_string()),
        (0x21, "Y1T".to_string()),
        (0x22, "X2T".to_string()),
        (0x23, "Y2T".to_string()),
        (0x24, "X3T".to_string()),
        (0x25, "Y3T".to_string()),
        (0x26, "X4T".to_string()),
        (0x27, "Y4T".to_string()),
        (0x28, "X5T".to_string()),
        (0x29, "Y5T".to_string()),
        (0x2A, "X6T".to_string()),
        (0x2B, "Y6T".to_string()),
        (0x40, "R1".to_string()),
        (0x41, "R2".to_string()),
        (0x42, "R3".to_string()),
        (0x43, "R4".to_string()),
        (0x44, "R5".to_string()),
        (0x45, "R6".to_string()),
        (0x80, "U".to_string()),
        (0x81, "V".to_string()),
        (0x82, "W".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    tokens
}
