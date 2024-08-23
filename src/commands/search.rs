use crate::calculator::errors::TiToolsError;
use crate::calculator::tokens::{load_token_definitions, load_tokens, OsVersion};
use crate::calculator::DisplayMode;
use crate::prints;

#[derive(Debug, PartialEq)]
enum SearchTokenType {
    Accessible,
    Pretty,
    Byte,
}

impl SearchTokenType {
    pub fn from_string(search_token_type: &str) -> Result<SearchTokenType, TiToolsError> {
        match search_token_type {
            "accessible" => Ok(SearchTokenType::Accessible),
            "pretty" => Ok(SearchTokenType::Pretty),
            "byte" => Ok(SearchTokenType::Byte),
            _ => Err(TiToolsError::Match(
                search_token_type.to_string(),
                "Search Token Type".to_string(),
            )),
        }
    }
}

pub fn search_command(token: String, token_type_string: String) {
    let token_type = match SearchTokenType::from_string(&token_type_string) {
        Ok(token_type) => token_type,
        Err(err) => err.print().exit(),
    };

    if token_type == SearchTokenType::Pretty {
        prints!("[color:bright-yellow]Warning:[color:reset] Pretty tokens are less accurate for best results use accessible")
    }

    let tokens = match load_tokens(&OsVersion::latest()) {
        Ok(tokens) => tokens,
        Err(err) => err.print().exit(),
    };

    let byte = if token_type != SearchTokenType::Byte {
        if token_type == SearchTokenType::Accessible {
            match tokens.get_longest_matching_token(&token, &DisplayMode::Accessible) {
                Some(byte) => fix_byte_format(&byte.0),
                None => {
                    prints!("[color:bright-red]Error:[color:reset] Failed to find token");
                    std::process::exit(1);
                }
            }
        } else if token_type == SearchTokenType::Pretty {
            match tokens.get_longest_matching_token(&token, &DisplayMode::Pretty) {
                Some(byte) => fix_byte_format(&byte.0),
                None => {
                    prints!("[color:bright-red]Error:[color:reset] Failed to find token");
                    std::process::exit(1);
                }
            }
        } else {
            prints!("[color:bright-red]Error:[color:reset] [color:bright-cyan]{}[color:reset] is an invalid token type", token_type_string);
            std::process::exit(1);
        }
    } else {
        if !token.starts_with("0x") {
            prints!("[color:bright-red]Error:[color:reset] Invalid byte format, Single Byte Token: 0x00, Two Byte Token: 0x0000");
            std::process::exit(1);
        };

        token
    };

    let token_definitions = match load_token_definitions() {
        Ok(token_definitions) => token_definitions,
        Err(err) => err.print().exit(),
    };

    let token_definition = match token_definitions.get(&byte) {
        Some(token_definition) => token_definition,
        None => {
            prints!("[color:bright-red]Error:[color:reset] Failed to find definition for token");
            std::process::exit(1);
        }
    };

    println!();
    for syntax_variation in token_definition {
        println!("syntax: {}", syntax_variation.syntax);
        println!("{}\n", syntax_variation.description);
    }
}

fn fix_byte_format(byte: &str) -> String {
    let byte_without_lang = byte.replace(" en", "");

    if byte.len() <= 3 {
        byte_without_lang.replace("$", "0x")
    } else {
        // $00$00
        let mut byte_split: Vec<&str> = byte_without_lang.split("").collect();
        byte_split.remove(4);
        // 0x0000
        byte_split.join("").replace("$", "0x")
    }
}
