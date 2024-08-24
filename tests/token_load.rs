use ti_tools::calculator::tokens::{load_token_definitions, load_tokens, OsVersion};

#[test]
fn test_load_token_definitions() {
    let tokens = load_token_definitions();

    assert!(tokens.is_ok(), "Failed to load token definitions.");
}

#[test]
fn test_load_tokens() {
    let tokens = load_tokens(&OsVersion::latest());

    assert!(tokens.is_ok(), "Failed to load tokens.");
}
