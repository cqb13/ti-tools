use std::collections::HashMap;

pub fn get_equation_tokens() -> HashMap<u8, String> {
    let tokens: HashMap<u8, String> = [
        (0x10, "{Y₁}".to_string()),
        (0x11, "{Y₂}".to_string()),
        (0x12, "{Y₃}".to_string()),
        (0x13, "{Y₄}".to_string()),
        (0x14, "{Y₅}".to_string()),
        (0x15, "{Y₆}".to_string()),
        (0x16, "{Y₇}".to_string()),
        (0x17, "{Y₈}".to_string()),
        (0x18, "{Y₉}".to_string()),
        (0x19, "{Y₀}".to_string()),
        (0x20, "X₁ᴛ".to_string()),
        (0x21, "Y₁ᴛ".to_string()),
        (0x22, "X₂ᴛ".to_string()),
        (0x23, "Y₂ᴛ".to_string()),
        (0x24, "X₃ᴛ".to_string()),
        (0x25, "Y₃ᴛ".to_string()),
        (0x26, "X₄ᴛ".to_string()),
        (0x27, "Y₄ᴛ".to_string()),
        (0x28, "X₅ᴛ".to_string()),
        (0x29, "Y₅ᴛ".to_string()),
        (0x2A, "X₆ᴛ".to_string()),
        (0x2B, "Y₆ᴛ".to_string()),
        (0x40, "r₁".to_string()),
        (0x41, "r₂".to_string()),
        (0x42, "r₃".to_string()),
        (0x43, "r₄".to_string()),
        (0x44, "r₅".to_string()),
        (0x45, "r₆".to_string()),
        (0x80, "{u}".to_string()),
        (0x81, "{v}".to_string()),
        (0x82, "{w}".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    tokens
}
