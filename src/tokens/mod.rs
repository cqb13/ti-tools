pub mod equation_tokens;
pub mod gdb_tokens;
pub mod graph_format_tokens;
pub mod list_tokens;
pub mod matrix_tokens;
pub mod misc_tokens;
pub mod picture_tokens;
pub mod single_byte_tokens;
pub mod statistic_variable_tokens;
pub mod string_tokens;
pub mod ti_84_tokens;

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
    pub list_tokens: HashMap<u8, String>,
    pub matrix_tokens: HashMap<u8, String>,
    pub misc_tokens: HashMap<u8, String>,
    pub picture_tokens: HashMap<u8, String>,
    pub single_byte_tokens: HashMap<u8, String>,
    pub statistic_variable_tokens: HashMap<u8, String>,
    pub string_tokens: HashMap<u8, String>,
    pub ti_84_tokens: HashMap<u8, String>,
}

impl AllTokens {
    pub fn new() -> AllTokens {
        AllTokens {
            equation_tokens: equation_tokens::get_equation_tokens(),
            gdb_tokens: gdb_tokens::get_gdb_tokens(),
            graph_format_tokens: graph_format_tokens::get_graph_format_tokens(),
            list_tokens: list_tokens::get_list_tokens(),
            matrix_tokens: matrix_tokens::get_matrix_tokens(),
            misc_tokens: misc_tokens::get_misc_tokens(),
            picture_tokens: picture_tokens::get_picture_tokens(),
            single_byte_tokens: single_byte_tokens::get_single_byte_tokens(),
            statistic_variable_tokens: statistic_variable_tokens::get_statistic_variable_tokens(),
            string_tokens: string_tokens::get_string_tokens(),
            ti_84_tokens: ti_84_tokens::get_ti_84_tokens(),
        }
    }
}
