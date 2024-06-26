pub mod equation_tokens;
pub mod gdb_tokens;
pub mod graph_format_tokens;
pub mod single_byte_tokens;

use std::collections::HashMap;

pub fn reverse_token_lookup(matrix_tokens: &HashMap<String, u8>) -> HashMap<String, u8> {
    let mut reverse_matrix_tokens: HashMap<String, u8> = HashMap::new();
    for (key, value) in matrix_tokens {
        reverse_matrix_tokens.insert(key.to_string(), *value);
    }

    reverse_matrix_tokens
}

pub struct AllTokens {
    pub equation_tokens: HashMap<u8, String>,
    pub gdb_tokens: HashMap<u8, String>,
    pub graph_format_tokens: HashMap<u8, String>,
    pub single_byte_tokens: HashMap<u8, String>,
}

impl AllTokens {
    pub fn new() -> AllTokens {
        AllTokens {
            equation_tokens: equation_tokens::get_equation_tokens(),
            gdb_tokens: gdb_tokens::get_gdb_tokens(),
            graph_format_tokens: graph_format_tokens::get_graph_format_tokens(),
            single_byte_tokens: single_byte_tokens::get_single_byte_tokens(),
        }
    }
}