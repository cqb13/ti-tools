pub mod accessible_to_display_tokens;
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
pub mod window_and_finance_tokens;

use std::collections::HashMap;

pub fn reverse_tokens(tokens: &HashMap<u8, String>) -> HashMap<String, u8> {
    let mut reversed_tokens: HashMap<String, u8> = HashMap::new();
    for (key, value) in tokens.iter() {
        reversed_tokens.insert(value.clone(), *key);
    }
    reversed_tokens
}

pub struct Tokens {
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
    pub window_and_finance_tokens: HashMap<u8, String>,
}

impl Tokens {
    pub fn new() -> Tokens {
        Tokens {
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
            window_and_finance_tokens: window_and_finance_tokens::get_window_and_finance_tokens(),
        }
    }
}

pub struct TokensReversed {
    pub equation_tokens: HashMap<String, u8>,
    pub gdb_tokens: HashMap<String, u8>,
    pub graph_format_tokens: HashMap<String, u8>,
    pub list_tokens: HashMap<String, u8>,
    pub matrix_tokens: HashMap<String, u8>,
    pub misc_tokens: HashMap<String, u8>,
    pub picture_tokens: HashMap<String, u8>,
    pub single_byte_tokens: HashMap<String, u8>,
    pub statistic_variable_tokens: HashMap<String, u8>,
    pub string_tokens: HashMap<String, u8>,
    pub ti_84_tokens: HashMap<String, u8>,
    pub window_and_finance_tokens: HashMap<String, u8>,
}

impl TokensReversed {
    pub fn new() -> TokensReversed {
        TokensReversed {
            equation_tokens: reverse_tokens(&equation_tokens::get_equation_tokens()),
            gdb_tokens: reverse_tokens(&gdb_tokens::get_gdb_tokens()),
            graph_format_tokens: reverse_tokens(&graph_format_tokens::get_graph_format_tokens()),
            list_tokens: reverse_tokens(&list_tokens::get_list_tokens()),
            matrix_tokens: reverse_tokens(&matrix_tokens::get_matrix_tokens()),
            misc_tokens: reverse_tokens(&misc_tokens::get_misc_tokens()),
            picture_tokens: reverse_tokens(&picture_tokens::get_picture_tokens()),
            single_byte_tokens: reverse_tokens(&single_byte_tokens::get_single_byte_tokens()),
            statistic_variable_tokens: reverse_tokens(
                &statistic_variable_tokens::get_statistic_variable_tokens(),
            ),
            string_tokens: reverse_tokens(&string_tokens::get_string_tokens()),
            ti_84_tokens: reverse_tokens(&ti_84_tokens::get_ti_84_tokens()),
            window_and_finance_tokens: reverse_tokens(
                &window_and_finance_tokens::get_window_and_finance_tokens(),
            ),
        }
    }
}
