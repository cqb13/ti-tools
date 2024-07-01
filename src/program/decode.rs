use super::DisplayMode;
use crate::tokens::Trie;

pub fn decode(
    bytestream: &[u8],
    trie: &Trie,
    lang: &str,
    mode: DisplayMode,
) -> Result<String, String> {
    let mut out = String::new();

    let mut index = 0;
    let mut current_bytes = Vec::new();

    while index < bytestream.len() {
        current_bytes.push(bytestream[index]);

        let key = current_bytes
            .iter()
            .map(|b| format!("${:02X}", b))
            .collect::<String>();

        if let Some(tokens) = trie.search(&key) {
            if let Some(token) = tokens.get(0) {
                let translation = token.langs.get(lang).ok_or("Language not found")?;
                let representation = match mode {
                    DisplayMode::Pretty => &translation.display,
                    DisplayMode::Accessible => &translation.accessible,
                    DisplayMode::TiAscii => &translation.ti_ascii,
                };
                out.push_str(representation);
                current_bytes.clear();
            }
        }

        index += 1;
    }

    if current_bytes.is_empty() {
        Ok(out)
    } else {
        Err(format!("Token not found: {:02X?}", current_bytes))
    }
}
