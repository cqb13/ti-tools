use std::collections::HashMap;

pub fn get_graph_format_tokens() -> HashMap<u8, String> {
    let tokens: HashMap<u8, String> = [
        (0x00, "Sequential".to_string()),
        (0x01, "Simul".to_string()),
        (0x02, "PolarGC".to_string()),
        (0x03, "RectGC".to_string()),
        (0x04, "CoordOn".to_string()),
        (0x05, "CoordOff".to_string()),
        (0x06, "Thick".to_string()),
        (0x07, "Dot-Thick".to_string()),
        (0x08, "AxesOn ".to_string()),
        (0x09, "AxesOff".to_string()),
        (0x0A, "GridDot ".to_string()),
        (0x0B, "GridOff".to_string()),
        (0x0C, "LabelOn".to_string()),
        (0x0D, "LabelOff".to_string()),
        (0x0E, "Web".to_string()),
        (0x0F, "Time".to_string()),
        (0x10, "uvAxes".to_string()),
        (0x11, "vwAxes".to_string()),
        (0x12, "uwAxes".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    tokens
}
