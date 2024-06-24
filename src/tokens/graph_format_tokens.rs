use std::collections::HashMap;

/**
 *
 *
 * const std::unordered_map<char, std::wstring> graph_format_tokens = {
  { 0x00, L"Sequential" },
  { 0x01, L"Simul" },
  { 0x02, L"PolarGC" },
  { 0x03, L"RectGC" },
  { 0x04, L"CoordOn" },
  { 0x05, L"CoordOff" },
  { 0x06, L"Connected" },
  { 0x07, L"Dot" },
  { 0x08, L"AxesOn" },
  { 0x09, L"AxesOff" },
  { 0x0A, L"GridOn" },
  { 0x0B, L"GridOff" },
  { 0x0C, L"LabelOn" },
  { 0x0D, L"LabelOff" },
  { 0x0E, L"Web" },
  { 0x0F, L"Time" },
  { 0x10, L"uvAxes" },
  { 0x11, L"vwAxes" },
  { 0x12, L"uwAxes" }
};

 */

pub enum GraphFormatToken {
    Sequential,
    Simul,
    PolarGC,
    RectGC,
    CoordOn,
    CoordOff,
    Connected,
    Dot,
    AxesOn,
    AxesOff,
    GridOn,
    GridOff,
    LabelOn,
    LabelOff,
    Web,
    Time,
    UvAxes,
    VwAxes,
    UwAxes,
}

impl GraphFormatToken {
    pub fn to_string(&self) -> String {
        match self {
            GraphFormatToken::Sequential => "Sequential".to_string(),
            GraphFormatToken::Simul => "Simul".to_string(),
            GraphFormatToken::PolarGC => "PolarGC".to_string(),
            GraphFormatToken::RectGC => "RectGC".to_string(),
            GraphFormatToken::CoordOn => "CoordOn".to_string(),
            GraphFormatToken::CoordOff => "CoordOff".to_string(),
            GraphFormatToken::Connected => "Connected".to_string(),
            GraphFormatToken::Dot => "Dot".to_string(),
            GraphFormatToken::AxesOn => "AxesOn".to_string(),
            GraphFormatToken::AxesOff => "AxesOff".to_string(),
            GraphFormatToken::GridOn => "GridOn".to_string(),
            GraphFormatToken::GridOff => "GridOff".to_string(),
            GraphFormatToken::LabelOn => "LabelOn".to_string(),
            GraphFormatToken::LabelOff => "LabelOff".to_string(),
            GraphFormatToken::Web => "Web".to_string(),
            GraphFormatToken::Time => "Time".to_string(),
            GraphFormatToken::UvAxes => "uvAxes".to_string(),
            GraphFormatToken::VwAxes => "vwAxes".to_string(),
            GraphFormatToken::UwAxes => "uwAxes".to_string(),
        }
    }
}

pub struct GraphFormatTokens {
    tokens: HashMap<u8, GraphFormatToken>,
}

impl GraphFormatTokens {
    pub fn new() -> GraphFormatTokens {
        let mut graph_format_tokens = HashMap::new();

        graph_format_tokens.insert(0x00, GraphFormatToken::Sequential);
        graph_format_tokens.insert(0x01, GraphFormatToken::Simul);
        graph_format_tokens.insert(0x02, GraphFormatToken::PolarGC);
        graph_format_tokens.insert(0x03, GraphFormatToken::RectGC);
        graph_format_tokens.insert(0x04, GraphFormatToken::CoordOn);
        graph_format_tokens.insert(0x05, GraphFormatToken::CoordOff);
        graph_format_tokens.insert(0x06, GraphFormatToken::Connected);
        graph_format_tokens.insert(0x07, GraphFormatToken::Dot);
        graph_format_tokens.insert(0x08, GraphFormatToken::AxesOn);
        graph_format_tokens.insert(0x09, GraphFormatToken::AxesOff);
        graph_format_tokens.insert(0x0A, GraphFormatToken::GridOn);
        graph_format_tokens.insert(0x0B, GraphFormatToken::GridOff);
        graph_format_tokens.insert(0x0C, GraphFormatToken::LabelOn);
        graph_format_tokens.insert(0x0D, GraphFormatToken::LabelOff);
        graph_format_tokens.insert(0x0E, GraphFormatToken::Web);
        graph_format_tokens.insert(0x0F, GraphFormatToken::Time);
        graph_format_tokens.insert(0x10, GraphFormatToken::UvAxes);
        graph_format_tokens.insert(0x11, GraphFormatToken::VwAxes);
        graph_format_tokens.insert(0x12, GraphFormatToken::UwAxes);

        GraphFormatTokens {
            tokens: graph_format_tokens,
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
