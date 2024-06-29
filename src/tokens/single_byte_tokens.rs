use std::collections::HashMap;

pub fn get_single_byte_tokens() -> HashMap<u8, String> {
    let tokens: HashMap<u8, String> = [
        (0x01, ">DMS".to_string()),
        (0x02, ">Dec".to_string()),
        (0x03, ">Frac".to_string()),
        (0x04, "->".to_string()),
        (0x05, "Boxplot".to_string()),
        (0x06, "[".to_string()),
        (0x07, "]".to_string()),
        (0x08, "{".to_string()),
        (0x09, "}".to_string()),
        (0x0A, "^^r".to_string()),
        (0x0B, "^^o".to_string()),
        (0x0C, "^^-1".to_string()),
        (0x0D, "^^2".to_string()),
        (0x0E, "^^T".to_string()),
        (0x0F, "^^3".to_string()),
        (0x10, "(".to_string()),
        (0x11, ")".to_string()),
        (0x12, "round(".to_string()),
        (0x13, "pxl-Test(".to_string()),
        (0x14, "augment(".to_string()),
        (0x15, "rowSwap(".to_string()),
        (0x16, "row+(".to_string()),
        (0x17, "*row(".to_string()),
        (0x18, "*row+(".to_string()),
        (0x19, "max(".to_string()),
        (0x1A, "min(".to_string()),
        (0x1B, "R>Pr(".to_string()),
        (0x1C, "R>Ptheta(".to_string()),
        (0x1D, "P>Rx(".to_string()),
        (0x1E, "P>Ry(".to_string()),
        (0x1F, "median(".to_string()),
        (0x20, "randM(".to_string()),
        (0x21, "mean(".to_string()),
        (0x22, "solve(".to_string()),
        (0x23, "seq(".to_string()),
        (0x24, "fnInt(".to_string()),
        (0x25, "nDeriv(".to_string()),
        (0x27, "fMin(".to_string()),
        (0x28, "fMax(".to_string()),
        (0x29, " ".to_string()),
        (0x2A, "\"".to_string()),
        (0x2B, ",".to_string()),
        (0x2C, "[i]".to_string()),
        (0x2D, "!".to_string()),
        (0x2E, "CubicReg ".to_string()),
        (0x2F, "QuartReg ".to_string()),
        (0x30, "0".to_string()),
        (0x31, "1".to_string()),
        (0x32, "2".to_string()),
        (0x33, "3".to_string()),
        (0x34, "4".to_string()),
        (0x35, "5".to_string()),
        (0x36, "6".to_string()),
        (0x37, "7".to_string()),
        (0x38, "8".to_string()),
        (0x39, "9".to_string()),
        (0x3A, ".".to_string()),
        (0x3B, "|E".to_string()),
        (0x3C, " or ".to_string()),
        (0x3D, " xor ".to_string()),
        (0x3E, ":".to_string()),
        (0x3F, "\n".to_string()),
        (0x40, " and ".to_string()),
        (0x41, "A".to_string()),
        (0x42, "B".to_string()),
        (0x43, "C".to_string()),
        (0x44, "D".to_string()),
        (0x45, "E".to_string()),
        (0x46, "F".to_string()),
        (0x47, "G".to_string()),
        (0x48, "H".to_string()),
        (0x49, "I".to_string()),
        (0x4A, "J".to_string()),
        (0x4B, "K".to_string()),
        (0x4C, "L".to_string()),
        (0x4D, "M".to_string()),
        (0x4E, "N".to_string()),
        (0x4F, "O".to_string()),
        (0x50, "P".to_string()),
        (0x51, "Q".to_string()),
        (0x52, "R".to_string()),
        (0x53, "S".to_string()),
        (0x54, "T".to_string()),
        (0x55, "U".to_string()),
        (0x56, "V".to_string()),
        (0x57, "W".to_string()),
        (0x58, "X".to_string()),
        (0x59, "Y".to_string()),
        (0x5A, "Z".to_string()),
        (0x5B, "theta".to_string()),
        (0x5C, "[error: unknown 2-byte code]".to_string()),
        (0x5D, "[error: unknown 2-byte code]".to_string()),
        (0x5E, "[error: unknown 2-byte code]".to_string()),
        (0x5F, "prgm".to_string()),
        (0x60, "[error: unknown 2-byte code]".to_string()),
        (0x61, "[error: unknown 2-byte code]".to_string()),
        (0x62, "[error: unknown 2-byte code]".to_string()),
        (0x63, "[error: unknown 2-byte code]".to_string()),
        (0x64, "Radian".to_string()),
        (0x65, "Degree".to_string()),
        (0x66, "Normal".to_string()),
        (0x67, "Sci".to_string()),
        (0x68, "Eng".to_string()),
        (0x69, "Float".to_string()),
        (0x6A, "=".to_string()),
        (0x6B, "<".to_string()),
        (0x6C, ">".to_string()),
        (0x6D, "<=".to_string()),
        (0x6E, ">=".to_string()),
        (0x6F, "!=".to_string()),
        (0x70, "+".to_string()),
        (0x71, "-".to_string()),
        (0x72, "Ans".to_string()),
        (0x73, "Fix ".to_string()),
        (0x74, "Horiz".to_string()),
        (0x75, "Full".to_string()),
        (0x76, "Func".to_string()),
        (0x77, "Param".to_string()),
        (0x78, "Polar".to_string()),
        (0x79, "Seq".to_string()),
        (0x7A, "IndpntAuto".to_string()),
        (0x7B, "IndpntAsk".to_string()),
        (0x7C, "DependAuto".to_string()),
        (0x7D, "DependAsk".to_string()),
        (0x7E, "[error: unknown 2-byte code]".to_string()),
        (0x7F, "squareplot".to_string()),
        (0x80, "crossplot".to_string()),
        (0x81, "dotplot".to_string()),
        (0x82, "*".to_string()),
        (0x83, "/".to_string()),
        (0x84, "Trace".to_string()),
        (0x85, "ClrDraw".to_string()),
        (0x86, "ZStandard".to_string()),
        (0x87, "ZTrig".to_string()),
        (0x88, "ZBox".to_string()),
        (0x89, "Zoom In".to_string()),
        (0x8A, "Zoom Out".to_string()),
        (0x8B, "ZSquare".to_string()),
        (0x8C, "ZInteger".to_string()),
        (0x8D, "ZPrevious".to_string()),
        (0x8E, "ZDecimal".to_string()),
        (0x8F, "ZoomStat".to_string()),
        (0x90, "ZoomRcl".to_string()),
        (0x91, "PrintScreen".to_string()),
        (0x92, "ZoomSto".to_string()),
        (0x93, "Text(".to_string()),
        (0x94, " nPr ".to_string()),
        (0x95, " nCr ".to_string()),
        (0x96, "FnOn ".to_string()),
        (0x97, "FnOff ".to_string()),
        (0x98, "StorePic ".to_string()),
        (0x99, "RecallPic ".to_string()),
        (0x9A, "StoreGDB ".to_string()),
        (0x9B, "RecallGDB ".to_string()),
        (0x9C, "Line(".to_string()),
        (0x9D, "Vertical ".to_string()),
        (0x9E, "Pt-On(".to_string()),
        (0x9F, "Pt-Off(".to_string()),
        (0xA0, "Pt-Change(".to_string()),
        (0xA1, "Pxl-On(".to_string()),
        (0xA2, "Pxl-Off(".to_string()),
        (0xA3, "Pxl-Change(".to_string()),
        (0xA4, "Shade(".to_string()),
        (0xA5, "Circle(".to_string()),
        (0xA6, "Horizontal ".to_string()),
        (0xA7, "Tangent(".to_string()),
        (0xA8, "DrawInv ".to_string()),
        (0xA9, "DrawF ".to_string()),
        (0xAA, "[error: unknown 2-byte code]".to_string()),
        (0xAB, "rand".to_string()),
        (0xAC, "pi".to_string()),
        (0xAD, "getKey".to_string()),
        (0xAE, "\'".to_string()),
        (0xAF, "?".to_string()),
        (0xB0, "~".to_string()),
        (0xB1, "int(".to_string()),
        (0xB2, "abs(".to_string()),
        (0xB3, "det(".to_string()),
        (0xB4, "identity(".to_string()),
        (0xB5, "dim(".to_string()),
        (0xB6, "sum(".to_string()),
        (0xB7, "prod(".to_string()),
        (0xB8, "not(".to_string()),
        (0xB9, "iPart(".to_string()),
        (0xBA, "fPart(".to_string()),
        (0xBB, "[error: unknown 2-byte code]".to_string()),
        (0xBC, "sqrt(".to_string()),
        (0xBD, "cuberoot(".to_string()),
        (0xBE, "ln(".to_string()),
        (0xBF, "e^^(".to_string()),
        (0xC0, "log(".to_string()),
        (0xC1, "10^^(".to_string()),
        (0xC2, "sin(".to_string()),
        (0xC3, "sin^-1(".to_string()),
        (0xC4, "cos(".to_string()),
        (0xC5, "cos^-1(".to_string()),
        (0xC6, "tan(".to_string()),
        (0xC7, "tan^-1(".to_string()),
        (0xC8, "sinh(".to_string()),
        (0xC9, "sinh^-1(".to_string()),
        (0xCA, "cosh(".to_string()),
        (0xCB, "cosh^-1(".to_string()),
        (0xCC, "tanh(".to_string()),
        (0xCD, "tanh^-1(".to_string()),
        (0xCE, "If ".to_string()),
        (0xCF, "Then".to_string()),
        (0xD0, "Else".to_string()),
        (0xD1, "While ".to_string()),
        (0xD2, "Repeat ".to_string()),
        (0xD3, "For(".to_string()),
        (0xD4, "End".to_string()),
        (0xD5, "Return".to_string()),
        (0xD6, "Lbl ".to_string()),
        (0xD7, "Goto ".to_string()),
        (0xD8, "Pause ".to_string()),
        (0xD9, "Stop".to_string()),
        (0xDA, "IS>(".to_string()),
        (0xDB, "DS<(".to_string()),
        (0xDC, "Input ".to_string()),
        (0xDD, "Prompt ".to_string()),
        (0xDE, "Disp ".to_string()),
        (0xDF, "DispGraph".to_string()),
        (0xE0, "Output(".to_string()),
        (0xE1, "ClrHome".to_string()),
        (0xE2, "Fill(".to_string()),
        (0xE3, "SortA(".to_string()),
        (0xE4, "SortD(".to_string()),
        (0xE5, "DispTable".to_string()),
        (0xE6, "Menu(".to_string()),
        (0xE7, "Send(".to_string()),
        (0xE8, "Get(".to_string()),
        (0xE9, "PlotsOn ".to_string()),
        (0xEA, "PlotsOff ".to_string()),
        (0xEB, "smallL".to_string()),
        (0xEC, "Plot1(".to_string()),
        (0xED, "Plot2(".to_string()),
        (0xEE, "Plot3(".to_string()),
        (0xEF, "[error: unknown 2-byte code]".to_string()),
        (0xF0, "^".to_string()),
        (0xF1, "xroot".to_string()),
        (0xF2, "1-Var Stats ".to_string()),
        (0xF3, "2-Var Stats ".to_string()),
        (0xF4, "LinReg(a+bx) ".to_string()),
        (0xF5, "ExpReg ".to_string()),
        (0xF6, "LnReg ".to_string()),
        (0xF7, "PwrReg ".to_string()),
        (0xF8, "Med-Med ".to_string()),
        (0xF9, "QuadReg ".to_string()),
        (0xFA, "ClrList ".to_string()),
        (0xFB, "ClrTable".to_string()),
        (0xFC, "Histogram".to_string()),
        (0xFD, "xyLine".to_string()),
        (0xFE, "Scatter".to_string()),
        (0xFF, "LinReg(ax+b) ".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    tokens
}
