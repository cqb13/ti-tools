use std::collections::HashMap;

pub enum GDBToken {
    GDB1,
    GDB2,
    GDB3,
    GDB4,
    GDB5,
    GDB6,
    GDB7,
    GDB8,
    GDB9,
    GDB0,
}

impl GDBToken {
    pub fn to_string(&self) -> String {
        match self {
            GDBToken::GDB1 => "GDB1".to_string(),
            GDBToken::GDB2 => "GDB2".to_string(),
            GDBToken::GDB3 => "GDB3".to_string(),
            GDBToken::GDB4 => "GDB4".to_string(),
            GDBToken::GDB5 => "GDB5".to_string(),
            GDBToken::GDB6 => "GDB6".to_string(),
            GDBToken::GDB7 => "GDB7".to_string(),
            GDBToken::GDB8 => "GDB8".to_string(),
            GDBToken::GDB9 => "GDB9".to_string(),
            GDBToken::GDB0 => "GDB0".to_string(),
        }
    }
}

pub struct GDBTokens {
    tokens: HashMap<u8, GDBToken>,
}

impl GDBTokens {
    pub fn new() -> GDBTokens {
        let mut gdb_tokens = HashMap::new();

        gdb_tokens.insert(0x00, GDBToken::GDB1);
        gdb_tokens.insert(0x01, GDBToken::GDB2);
        gdb_tokens.insert(0x02, GDBToken::GDB3);
        gdb_tokens.insert(0x03, GDBToken::GDB4);
        gdb_tokens.insert(0x04, GDBToken::GDB5);
        gdb_tokens.insert(0x05, GDBToken::GDB6);
        gdb_tokens.insert(0x06, GDBToken::GDB7);
        gdb_tokens.insert(0x07, GDBToken::GDB8);
        gdb_tokens.insert(0x08, GDBToken::GDB9);
        gdb_tokens.insert(0x09, GDBToken::GDB0);

        GDBTokens { tokens: gdb_tokens }
    }

    pub fn get_token_string(&self, byte: u8) -> Result<String, String> {
        let token_map = &self.tokens;
        let key_value = token_map.get_key_value(&byte);

        match key_value {
            Some((_, value)) => Ok(value.to_string()),
            None => Err(format!("No token is associated with {}", byte)),
        }
    }
}
