use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::program::DisplayMode;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OsVersion {
    pub model: String,
    #[serde(rename = "os-version")]
    pub version: String,
}

impl Ord for OsVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_order = model_order(&self.model);
        let other_order = model_order(&other.model);

        self_order.cmp(&other_order).then_with(|| {
            if self.version == "latest" {
                Ordering::Greater
            } else if other.version == "latest" {
                Ordering::Less
            } else if self.version.is_empty() {
                Ordering::Less
            } else if other.version.is_empty() {
                Ordering::Greater
            } else {
                self.version
                    .split('.')
                    .map(|s| s.parse::<u32>().unwrap_or(0))
                    .cmp(
                        other
                            .version
                            .split('.')
                            .map(|s| s.parse::<u32>().unwrap_or(0)),
                    )
            }
        })
    }
}

impl PartialOrd for OsVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn model_order(model: &str) -> u32 {
    match model {
        "" => 0,
        "TI-82" => 10,
        "TI-83" | "TI-82ST" | "TI-82ST.fr" | "TI-76.fr" => 20,
        "TI-83+" | "TI-83+SE" | "TI-83+.fr" | "TI-82+" => 30,
        "TI-84+" | "TI-84+SE" | "TI-83+.fr:USB" | "TI-84P.fr" | "TI-84+PSE" => 40,
        "TI-82A" | "TI-84+T" => 45,
        "TI-84+CSE" => 50,
        "TI-84+CE" | "TI-84+CET" | "TI-83PCE" | "TI-83PCEEP" | "TI-84+CEPY" | "TI-84+CETPE"
        | "TI-82AEP" => 60,
        "latest" => 9999999,
        _ => panic!("Unknown model"),
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Translation {
    #[serde(rename = "ti-ascii")]
    pub ti_ascii: String,
    pub display: String,
    pub accessible: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    pub since: OsVersion,
    pub until: Option<OsVersion>,
    pub langs: HashMap<String, Translation>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
enum TokenData {
    Single(Vec<Token>),
    Nested(std::collections::BTreeMap<String, Vec<Token>>),
}

pub struct Map {
    pub map: HashMap<String, Translation>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Translation) {
        self.map.insert(key, value);
    }

    pub fn get_value(&self, key: &str) -> Option<&Translation> {
        self.map.get(key)
    }

    pub fn get_token(&self, value: String, display_mode: &DisplayMode) -> Option<String> {
        let mut longest_key: &str = "";

        for (key, translation) in &self.map {
            let translation = match display_mode {
                DisplayMode::Pretty => &translation.display,
                DisplayMode::Accessible => &translation.accessible,
                DisplayMode::TiAscii => &translation.ti_ascii,
            };
            if translation == &value {
                longest_key = key;
            }
        }

        if longest_key.is_empty() {
            None
        } else {
            Some(longest_key.to_string())
        }
    }

    pub fn get_longest_matching_token(
        &self,
        value: &String,
        display_mode: &DisplayMode,
    ) -> Option<(String, String)> {
        let mut longest_key: &str = "";
        let mut longest_value = String::new();
        let mut longest_length = 0;

        for (token, translation) in &self.map {
            let translation = match display_mode {
                DisplayMode::Pretty => &translation.display,
                DisplayMode::Accessible => &translation.accessible,
                DisplayMode::TiAscii => &translation.ti_ascii,
            };

            if value.starts_with(translation) {
                if translation.len() > longest_length {
                    longest_key = token;
                    longest_value = translation.to_string();
                    longest_length = translation.len();
                }
            }
        }

        if longest_key.is_empty() {
            None
        } else {
            Some((longest_key.to_string(), longest_value))
        }
    }

    pub fn get_shortest_matching_token(&self, value: String, display_mode: &DisplayMode) -> String {
        let mut shortest_key: &str = "";
        let mut shortest_length = usize::MAX;

        for (key, translation) in &self.map {
            let translation = match display_mode {
                DisplayMode::Pretty => &translation.display,
                DisplayMode::Accessible => &translation.accessible,
                DisplayMode::TiAscii => &translation.ti_ascii,
            };

            if value.starts_with(translation) && translation.len() < shortest_length {
                shortest_key = key;
                shortest_length = translation.len();
            }
        }

        shortest_key.to_string()
    }
}

pub fn load_tokens(target: &OsVersion) -> Map {
    let json_data = include_str!("./standard_tokens/8X.json");

    let tokens: std::collections::BTreeMap<String, TokenData> =
        serde_json::from_str(json_data).unwrap();
    let mut map = Map::new();

    for (key, token_data) in tokens {
        match token_data {
            TokenData::Single(tokens) => {
                let new_tokens: Vec<Token> = tokens
                    .into_iter()
                    .filter(|token| token.since <= *target)
                    .filter(|token| {
                        token.until.is_none() || token.until.as_ref().unwrap() >= &target
                    })
                    .collect();

                for token in new_tokens {
                    for (lang, translation) in token.langs {
                        map.insert(format!("{} {}", key, lang), translation);
                    }
                }
            }
            TokenData::Nested(nested_tokens) => {
                for (sub_key, tokens) in nested_tokens {
                    let full_key = format!("{}{}", key, sub_key);
                    let new_tokens: Vec<Token> = tokens
                        .into_iter()
                        .filter(|token| token.since <= *target)
                        .filter(|token| {
                            token.until.is_none() || token.until.as_ref().unwrap() >= &target
                        })
                        .collect();
                    for token in new_tokens {
                        for (lang, translation) in token.langs {
                            map.insert(format!("{} {}", full_key, lang), translation);
                        }
                    }
                }
            }
        }
    }

    map
}
