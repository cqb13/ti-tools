use crate::calculator::errors::TiToolsError;
use crate::calculator::models::Model;
use crate::calculator::DisplayMode;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct OsVersion {
    pub model: Model,
    #[serde(rename = "os-version")]
    pub version: String,
}

impl OsVersion {
    pub fn new(model: Model, version: String) -> OsVersion {
        OsVersion { model, version }
    }

    pub fn latest() -> OsVersion {
        OsVersion {
            model: Model::Latest,
            version: "latest".to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for Model {
    fn deserialize<D>(deserializer: D) -> Result<Model, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(Model::from_string(&s))
    }
}

impl Ord for OsVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_order = self.model.model_order();
        let other_order = other.model.model_order();

        self_order.cmp(&other_order).then_with(|| {
            if self.version == "latest" {
                Ordering::Greater
            } else if other.version == "latest" || self.version.is_empty() {
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

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Translation {
    #[serde(rename = "ti-ascii")]
    pub ti_ascii: String,
    pub display: String,
    pub accessible: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Token {
    pub since: OsVersion,
    pub until: Option<OsVersion>,
    pub langs: HashMap<String, Translation>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum TokenData {
    Single(Vec<Token>),
    Nested(std::collections::BTreeMap<String, Vec<Token>>),
}

pub struct Map {
    pub map: HashMap<String, Translation>,
}

impl Map {
    pub fn new() -> Map {
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

    pub fn get_longest_matching_token(
        &self,
        value: &str,
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

            if value.starts_with(translation) && translation.len() > longest_length {
                longest_key = token;
                longest_value = translation.to_string();
                longest_length = translation.len();
            }
        }

        if longest_key.is_empty() {
            None
        } else {
            Some((longest_key.to_string(), longest_value))
        }
    }

    pub fn get_shortest_matching_token(
        &self,
        value: &str,
        display_mode: &DisplayMode,
    ) -> Option<(String, String)> {
        let mut shortest_key: &str = "";
        let mut shortest_value = String::new();
        let mut shortest_length = 9999;

        for (token, translation) in &self.map {
            let translation = match display_mode {
                DisplayMode::Pretty => &translation.display,
                DisplayMode::Accessible => &translation.accessible,
                DisplayMode::TiAscii => &translation.ti_ascii,
            };

            if value.starts_with(translation) && translation.len() < shortest_length {
                shortest_key = token;
                shortest_value = translation.to_string();
                shortest_length = translation.len();
            }
        }

        if shortest_key.is_empty() {
            None
        } else {
            Some((shortest_key.to_string(), shortest_value))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TokenDefinition {
    pub syntax: String,
    pub description: String,
}

pub fn load_token_definitions() -> Result<HashMap<String, Vec<TokenDefinition>>, TiToolsError> {
    let json_data = include_str!("./token_definitions.json");

    let token_definitions: HashMap<String, Vec<TokenDefinition>> =
        match serde_json::from_str(json_data) {
            Ok(token_definitions) => token_definitions,
            Err(err) => return Err(TiToolsError::Json(err.to_string())),
        };

    Ok(token_definitions)
}

pub fn load_tokens(target: &OsVersion) -> Result<Map, TiToolsError> {
    let json_data = include_str!("./standard_tokens/8X.json");

    let tokens: std::collections::BTreeMap<String, TokenData> =
        match serde_json::from_str(json_data) {
            Ok(tokens) => tokens,
            Err(err) => return Err(TiToolsError::Json(err.to_string())),
        };

    let mut map = Map::new();

    for (key, token_data) in tokens {
        match token_data {
            TokenData::Single(tokens) => {
                let new_tokens: Vec<Token> = tokens
                    .into_iter()
                    .filter(|token| token.since <= *target)
                    .filter(|token| {
                        token.until.is_none() || token.until.as_ref().unwrap() >= target
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
                            token.until.is_none() || token.until.as_ref().unwrap() >= target
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

    Ok(map)
}
