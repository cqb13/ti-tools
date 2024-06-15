use std::collections::HashMap;

#[derive(Clone)]
pub enum EquationToken {
    Y1,
    Y2,
    Y3,
    Y4,
    Y5,
    Y6,
    Y7,
    Y8,
    Y9,
    Y0,
    X1T,
    Y1T,
    X2T,
    Y2T,
    X3T,
    Y3T,
    X4T,
    Y4T,
    X5T,
    Y5T,
    X6T,
    Y6T,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    U,
    V,
    W,
}

impl EquationToken {
    pub fn to_string(&self) -> String {
        match self {
            EquationToken::Y1 => "Y1".to_string(),
            EquationToken::Y2 => "Y2".to_string(),
            EquationToken::Y3 => "Y3".to_string(),
            EquationToken::Y4 => "Y4".to_string(),
            EquationToken::Y5 => "Y5".to_string(),
            EquationToken::Y6 => "Y6".to_string(),
            EquationToken::Y7 => "Y7".to_string(),
            EquationToken::Y8 => "Y8".to_string(),
            EquationToken::Y9 => "Y9".to_string(),
            EquationToken::Y0 => "Y0".to_string(),
            EquationToken::X1T => "X1T".to_string(),
            EquationToken::Y1T => "X1T".to_string(),
            EquationToken::X2T => "X2T".to_string(),
            EquationToken::Y2T => "Y2T".to_string(),
            EquationToken::X3T => "X3T".to_string(),
            EquationToken::Y3T => "Y3T".to_string(),
            EquationToken::X4T => "X4T".to_string(),
            EquationToken::Y4T => "Y4T".to_string(),
            EquationToken::X5T => "X5T".to_string(),
            EquationToken::Y5T => "Y5T".to_string(),
            EquationToken::X6T => "X6T".to_string(),
            EquationToken::Y6T => "Y6T".to_string(),
            EquationToken::R1 => "R1".to_string(),
            EquationToken::R2 => "R2".to_string(),
            EquationToken::R3 => "R3".to_string(),
            EquationToken::R4 => "R4".to_string(),
            EquationToken::R5 => "R5".to_string(),
            EquationToken::R6 => "R6".to_string(),
            EquationToken::U => "U".to_string(),
            EquationToken::V => "V".to_string(),
            EquationToken::W => "W".to_string(),
        }
    }
}

pub struct EquationTokens {
    tokens: HashMap<u8, EquationToken>,
}

impl EquationTokens {
    pub fn new() -> EquationTokens {
        let mut equation_tokens = HashMap::new();

        equation_tokens.insert(0x10, EquationToken::Y1);
        equation_tokens.insert(0x11, EquationToken::Y2);
        equation_tokens.insert(0x12, EquationToken::Y3);
        equation_tokens.insert(0x13, EquationToken::Y4);
        equation_tokens.insert(0x14, EquationToken::Y5);
        equation_tokens.insert(0x15, EquationToken::Y6);
        equation_tokens.insert(0x16, EquationToken::Y7);
        equation_tokens.insert(0x17, EquationToken::Y8);
        equation_tokens.insert(0x18, EquationToken::Y9);
        equation_tokens.insert(0x19, EquationToken::Y0);
        equation_tokens.insert(0x20, EquationToken::X1T);
        equation_tokens.insert(0x21, EquationToken::Y1T);
        equation_tokens.insert(0x22, EquationToken::X2T);
        equation_tokens.insert(0x23, EquationToken::Y2T);
        equation_tokens.insert(0x24, EquationToken::X3T);
        equation_tokens.insert(0x25, EquationToken::Y3T);
        equation_tokens.insert(0x26, EquationToken::X4T);
        equation_tokens.insert(0x27, EquationToken::Y4T);
        equation_tokens.insert(0x28, EquationToken::X5T);
        equation_tokens.insert(0x29, EquationToken::Y5T);
        equation_tokens.insert(0x2A, EquationToken::X6T);
        equation_tokens.insert(0x2B, EquationToken::Y6T);
        equation_tokens.insert(0x40, EquationToken::R1);
        equation_tokens.insert(0x41, EquationToken::R2);
        equation_tokens.insert(0x42, EquationToken::R3);
        equation_tokens.insert(0x43, EquationToken::R4);
        equation_tokens.insert(0x44, EquationToken::R5);
        equation_tokens.insert(0x45, EquationToken::R6);
        equation_tokens.insert(0x80, EquationToken::U);
        equation_tokens.insert(0x81, EquationToken::V);
        equation_tokens.insert(0x82, EquationToken::W);

        EquationTokens {
            tokens: equation_tokens,
        }
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
