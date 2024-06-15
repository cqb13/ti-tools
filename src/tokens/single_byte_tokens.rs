use std::collections::HashMap;

pub enum SingleByteToken {
    UnusedCodePoint,
    R,
    DegreeSymbol,
    SuperscriptOne,
    SuperscriptTwo,
    T,
    SuperscriptThree,
    RightArrowDMS,
    OpenParenthesis,
    CloseParenthesis,
    Round,
    PxlTest,
    Augment,
    RowSwap,
    RowPlus,
    MultiplyRow,
    MultiplyRowPlus,
    Max,
    Min,
    RPr,
    RPTheta,
    PRx,
    PRy,
    Median,
    RightArrowDec,
    RandM,
    Mean,
    Solve,
    Sequence,
    FnInt,
    NDeriv,
    FMin,
    FMax,
    Space,
    QuotationMark,
    Comma,
    I,
    ExclamationMark,
    CubicReg,
    QuartReg,
    RightArrowFrac,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Period,
    E,
    Or,
    Xor,
    Colon,
    Newline,
    RightArrow,
    And,
    LetterA,
    LetterB,
    LetterC,
    LetterD,
    LetterE,
    LetterF,
    LetterG,
    LetterH,
    LetterI,
    LetterJ,
    LetterK,
    LetterL,
    LetterM,
    LetterN,
    LetterO,
    Boxplot,
    LetterP,
    LetterQ,
    LetterR,
    LetterS,
    LetterT,
    LetterU,
    LetterV,
    LetterW,
    LetterX,
    LetterY,
    LetterZ,
    Theta,
    Unknown2ByteCode,
    Prgm,
    OpenBracket,
    Radian,
    Degree,
    Normal,
    Sci,
    Eng,
    Float,
    Equals,
    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
    NotEqualTo,
    CloseBracket,
    Add,
    Subtract,
    Ans,
    Fix,
    Horiz,
    Full,
    Func,
    Param,
    Polar,
    Seq,
    IndpntAuto,
    IndpntAsk,
    DependAuto,
    DependAsk,
    SquareMark,
    OpenBrace,
    PlusMark,
    DotMark,
    Multiply,
    Divide,
    Trace,
    ClrDraw,
    ZStandard,
    ZTrig,
    ZBox,
    ZoomIn,
    ZoomOut,
    ZSquare,
    ZInteger,
    ZPrevious,
    ZDecimal,
    ZoomStat,
    CloseBrace,
    ZoomRcl,
    PrintScreen,
    ZoomSto,
    Text,
    NPr,
    NCr,
    FnOn,
    FnOff,
    StorePic,
    RecallPic,
    StoreGDB,
    RecallGDB,
    Line,
    Vertical,
    PtOn,
    PtOff,
    PtChange,
    PxlOn,
    PxlOff,
    PxlChange,
    Shade,
    Circle,
    Horizontal,
    Tangent,
    DrawInv,
    DrawF,
    Rand,
    Pi,
    GetKey,
    Apostrophe,
    QuestionMark,
    Negative,
    Int,
    Abs,
    Det,
    Identity,
    Dim,
    Sum,
    Prod,
    Not,
    IPart,
    FPart,
    SquareRoot,
    CubeRoot,
    NaturalLogarithm,
    Exponential,
    Logarithm,
    TenToThePowerOf,
    ShortSine,
    ShortSineInverse,
    ShortCosine,
    ShortCosineInverse,
    ShortTangent,
    ShortTangentInverse,
    ShortSineHyperbolic,
    ShortSineHyperbolicInverse,
    ShortCosineHyperbolic,
    ShortCosineHyperbolicInverse,
    ShortTangentHyperbolic,
    ShortTangentHyperbolicInverse,
    If,
    Then,
    Else,
    While,
    Repeat,
    For,
    End,
    Return,
    Lbl,
    Goto,
    Pause,
    Stop,
    IncrementSkipIfGreaterThan,
    DecrementSkipIfLessThan,
    Input,
    Prompt,
    Disp,
    DispGraph,
    Output,
    ClrHome,
    Fill,
    SortA,
    SortD,
    DispTable,
    Menu,
    Send,
    Get,
    PlotsOn,
    PlotsOff,
    Angle,
    Plot1,
    Plot2,
    Plot3,
    ExponentMark,
    CustomXSuperscriptRadical,
    OneVarStats,
    TwoVarStats,
    LinRegABPlusX,
    ExpReg,
    LnReg,
    PwrReg,
    MedMed,
    QuadReg,
    ClrList,
    ClrTable,
    Histogram,
    XYLine,
    Scatter,
    LinRegAXPlusB,
}

impl SingleByteToken {
    pub fn to_string(&self) -> String {
        match self {
            SingleByteToken::UnusedCodePoint => "[error: unused code point]".to_string(),
            SingleByteToken::R => "r".to_string(),
            SingleByteToken::DegreeSymbol => "\u{00B0}".to_string(),
            SingleByteToken::SuperscriptOne => "\u{05BF}\u{00B9}".to_string(),
            SingleByteToken::SuperscriptTwo => "\u{00B2}".to_string(),
            SingleByteToken::T => "T".to_string(),
            SingleByteToken::SuperscriptThree => "\u{00B3}".to_string(),
            SingleByteToken::RightArrowDMS => "\u{25BA}DMS".to_string(),
            SingleByteToken::OpenParenthesis => "(".to_string(),
            SingleByteToken::CloseParenthesis => ")".to_string(),
            SingleByteToken::Round => "round(".to_string(),
            SingleByteToken::PxlTest => "pxl-Test(".to_string(),
            SingleByteToken::Augment => "augment(".to_string(),
            SingleByteToken::RowSwap => "rowSwap(".to_string(),
            SingleByteToken::RowPlus => "row+(".to_string(),
            SingleByteToken::MultiplyRow => "*row(".to_string(),
            SingleByteToken::MultiplyRowPlus => "*row+(".to_string(),
            SingleByteToken::Max => "max(".to_string(),
            SingleByteToken::Min => "min(".to_string(),
            SingleByteToken::RPr => "R\u{25BA}Pr(".to_string(),
            SingleByteToken::RPTheta => "R\u{25BA}P\u{03B8}(".to_string(),
            SingleByteToken::PRx => "P\u{25BA}Rx(".to_string(),
            SingleByteToken::PRy => "P\u{25BA}Ry".to_string(),
            SingleByteToken::Median => "median(".to_string(),
            SingleByteToken::RightArrowDec => "\u{25BA}Dec".to_string(),
            SingleByteToken::RandM => "randM(".to_string(),
            SingleByteToken::Mean => "mean(".to_string(),
            SingleByteToken::Solve => "solve(".to_string(),
            SingleByteToken::Sequence => "seq(".to_string(),
            SingleByteToken::FnInt => "fnInt(".to_string(),
            SingleByteToken::NDeriv => "nDeriv(".to_string(),
            SingleByteToken::FMin => "fMin(".to_string(),
            SingleByteToken::FMax => "fMax(".to_string(),
            SingleByteToken::Space => " ".to_string(),
            SingleByteToken::QuotationMark => "\"".to_string(),
            SingleByteToken::Comma => ",".to_string(),
            SingleByteToken::I => "i".to_string(),
            SingleByteToken::ExclamationMark => "!".to_string(),
            SingleByteToken::CubicReg => "CubicReg".to_string(),
            SingleByteToken::QuartReg => "QuartReg".to_string(),
            SingleByteToken::RightArrowFrac => "\u{25BA}Frac".to_string(),
            SingleByteToken::Zero => "0".to_string(),
            SingleByteToken::One => "1".to_string(),
            SingleByteToken::Two => "2".to_string(),
            SingleByteToken::Three => "3".to_string(),
            SingleByteToken::Four => "4".to_string(),
            SingleByteToken::Five => "5".to_string(),
            SingleByteToken::Six => "6".to_string(),
            SingleByteToken::Seven => "7".to_string(),
            SingleByteToken::Eight => "8".to_string(),
            SingleByteToken::Nine => "9".to_string(),
            SingleByteToken::Period => ".".to_string(),
            SingleByteToken::E => "E".to_string(),
            SingleByteToken::Or => "or".to_string(),
            SingleByteToken::Xor => "xor".to_string(),
            SingleByteToken::Colon => ":".to_string(),
            SingleByteToken::Newline => "\n".to_string(),
            SingleByteToken::RightArrow => "\u{2192}".to_string(),
            SingleByteToken::And => "and".to_string(),
            SingleByteToken::LetterA => "A".to_string(),
            SingleByteToken::LetterB => "B".to_string(),
            SingleByteToken::LetterC => "C".to_string(),
            SingleByteToken::LetterD => "D".to_string(),
            SingleByteToken::LetterE => "E".to_string(),
            SingleByteToken::LetterF => "F".to_string(),
            SingleByteToken::LetterG => "G".to_string(),
            SingleByteToken::LetterH => "H".to_string(),
            SingleByteToken::LetterI => "I".to_string(),
            SingleByteToken::LetterJ => "J".to_string(),
            SingleByteToken::LetterK => "K".to_string(),
            SingleByteToken::LetterL => "L".to_string(),
            SingleByteToken::LetterM => "M".to_string(),
            SingleByteToken::LetterN => "N".to_string(),
            SingleByteToken::LetterO => "O".to_string(),
            SingleByteToken::Boxplot => "Boxplot".to_string(),
            SingleByteToken::LetterP => "P".to_string(),
            SingleByteToken::LetterQ => "Q".to_string(),
            SingleByteToken::LetterR => "R".to_string(),
            SingleByteToken::LetterS => "S".to_string(),
            SingleByteToken::LetterT => "T".to_string(),
            SingleByteToken::LetterU => "U".to_string(),
            SingleByteToken::LetterV => "V".to_string(),
            SingleByteToken::LetterW => "W".to_string(),
            SingleByteToken::LetterX => "X".to_string(),
            SingleByteToken::LetterY => "Y".to_string(),
            SingleByteToken::LetterZ => "Z".to_string(),
            SingleByteToken::Theta => "\u{03B8}".to_string(),
            SingleByteToken::Unknown2ByteCode => "[error: unknown 2-byte code]".to_string(),
            SingleByteToken::Prgm => "prgm".to_string(),
            SingleByteToken::OpenBracket => "[".to_string(),
            SingleByteToken::Radian => "Radian".to_string(),
            SingleByteToken::Degree => "Degree".to_string(),
            SingleByteToken::Normal => "Normal".to_string(),
            SingleByteToken::Sci => "Sci".to_string(),
            SingleByteToken::Eng => "Eng".to_string(),
            SingleByteToken::Float => "Float".to_string(),
            SingleByteToken::Equals => "=".to_string(),
            SingleByteToken::LessThan => "<".to_string(),
            SingleByteToken::GreaterThan => ">".to_string(),
            SingleByteToken::LessThanOrEqualTo => "\u{2264}".to_string(),
            SingleByteToken::GreaterThanOrEqualTo => "\u{2266}".to_string(),
            SingleByteToken::NotEqualTo => "\u{2260}".to_string(),
            SingleByteToken::CloseBracket => "]".to_string(),
            SingleByteToken::Add => "+".to_string(),
            SingleByteToken::Subtract => "-".to_string(),
            SingleByteToken::Ans => "Ans".to_string(),
            SingleByteToken::Fix => "Fix".to_string(),
            SingleByteToken::Horiz => "Horiz".to_string(),
            SingleByteToken::Full => "Full".to_string(),
            SingleByteToken::Func => "Func".to_string(),
            SingleByteToken::Param => "Param".to_string(),
            SingleByteToken::Polar => "Polar".to_string(),
            SingleByteToken::Seq => "Seq".to_string(),
            SingleByteToken::IndpntAuto => "IndpntAuto".to_string(),
            SingleByteToken::IndpntAsk => "IndpntAsk".to_string(),
            SingleByteToken::DependAuto => "DependAuto".to_string(),
            SingleByteToken::DependAsk => "DependAsk".to_string(),
            SingleByteToken::SquareMark => "[square mark]".to_string(),
            SingleByteToken::OpenBrace => "{".to_string(),
            SingleByteToken::PlusMark => "[plus mark]".to_string(),
            SingleByteToken::DotMark => "[dot mark]".to_string(),
            SingleByteToken::Multiply => "*".to_string(),
            SingleByteToken::Divide => "/".to_string(),
            SingleByteToken::Trace => "Trace".to_string(),
            SingleByteToken::ClrDraw => "ClrDraw".to_string(),
            SingleByteToken::ZStandard => "ZStandard".to_string(),
            SingleByteToken::ZTrig => "ZTrig".to_string(),
            SingleByteToken::ZBox => "ZBox".to_string(),
            SingleByteToken::ZoomIn => "Zoom In".to_string(),
            SingleByteToken::ZoomOut => "Zoom Out".to_string(),
            SingleByteToken::ZSquare => "ZSquare".to_string(),
            SingleByteToken::ZInteger => "ZInteger".to_string(),
            SingleByteToken::ZPrevious => "ZPrevious".to_string(),
            SingleByteToken::ZDecimal => "ZDecimal".to_string(),
            SingleByteToken::ZoomStat => "ZoomStat".to_string(),
            SingleByteToken::CloseBrace => "}".to_string(),
            SingleByteToken::ZoomRcl => "ZoomRcl".to_string(),
            SingleByteToken::PrintScreen => "PrintScreen".to_string(),
            SingleByteToken::ZoomSto => "ZoomSto".to_string(),
            SingleByteToken::Text => "Text(".to_string(),
            SingleByteToken::NPr => " nPr ".to_string(),
            SingleByteToken::NCr => " nCr ".to_string(),
            SingleByteToken::FnOn => "FnOn".to_string(),
            SingleByteToken::FnOff => "FnOff".to_string(),
            SingleByteToken::StorePic => "StorePic".to_string(),
            SingleByteToken::RecallPic => "RecallPic".to_string(),
            SingleByteToken::StoreGDB => "StoreGDB".to_string(),
            SingleByteToken::RecallGDB => "RecallGDB".to_string(),
            SingleByteToken::Line => "Line(".to_string(),
            SingleByteToken::Vertical => "Vertical".to_string(),
            SingleByteToken::PtOn => "Pt-On(".to_string(),
            SingleByteToken::PtOff => "Pt-Off(".to_string(),
            SingleByteToken::PtChange => "Pt-Change(".to_string(),
            SingleByteToken::PxlOn => "Pxl-On(".to_string(),
            SingleByteToken::PxlOff => "Pxl-Off(".to_string(),
            SingleByteToken::PxlChange => "Pxl-Change(".to_string(),
            SingleByteToken::Shade => "Shade(".to_string(),
            SingleByteToken::Circle => "Circle(".to_string(),
            SingleByteToken::Horizontal => "Horizontal".to_string(),
            SingleByteToken::Tangent => "Tangent(".to_string(),
            SingleByteToken::DrawInv => "DrawInv".to_string(),
            SingleByteToken::DrawF => "DrawF".to_string(),
            SingleByteToken::Rand => "rand".to_string(),
            SingleByteToken::Pi => "\u{03C0}".to_string(),
            SingleByteToken::GetKey => "getKey".to_string(),
            SingleByteToken::Apostrophe => "\'".to_string(),
            SingleByteToken::QuestionMark => "?".to_string(),
            SingleByteToken::Negative => "-".to_string(),
            SingleByteToken::Int => "int(".to_string(),
            SingleByteToken::Abs => "abs(".to_string(),
            SingleByteToken::Det => "det(".to_string(),
            SingleByteToken::Identity => "identity(".to_string(),
            SingleByteToken::Dim => "dim(".to_string(),
            SingleByteToken::Sum => "sum(".to_string(),
            SingleByteToken::Prod => "prod(".to_string(),
            SingleByteToken::Not => "not(".to_string(),
            SingleByteToken::IPart => "iPart(".to_string(),
            SingleByteToken::FPart => "fPart(".to_string(),
            SingleByteToken::SquareRoot => "\u{221A}(".to_string(),
            SingleByteToken::CubeRoot => "\u{00B3}\u{221A}(".to_string(),
            SingleByteToken::NaturalLogarithm => "ln(".to_string(),
            SingleByteToken::Exponential => "e^(".to_string(),
            SingleByteToken::Logarithm => "log(".to_string(),
            SingleByteToken::TenToThePowerOf => "10^(".to_string(),
            SingleByteToken::ShortSine => "sin(".to_string(),
            SingleByteToken::ShortSineInverse => "sin\u{05BF}\u{00B9}(".to_string(),
            SingleByteToken::ShortCosine => "cos(".to_string(),
            SingleByteToken::ShortCosineInverse => "cos\u{05BF}\u{00B9}(".to_string(),
            SingleByteToken::ShortTangent => "tan(".to_string(),
            SingleByteToken::ShortTangentInverse => "tan\u{05BF}\u{00B9}(".to_string(),
            SingleByteToken::ShortSineHyperbolic => "sinh(".to_string(),
            SingleByteToken::ShortSineHyperbolicInverse => "sinh\u{05BF}\u{00B9}(".to_string(),
            SingleByteToken::ShortCosineHyperbolic => "cosh(".to_string(),
            SingleByteToken::ShortCosineHyperbolicInverse => "cosh\u{05BF}\u{00B9}(".to_string(),
            SingleByteToken::ShortTangentHyperbolic => "tanh(".to_string(),
            SingleByteToken::ShortTangentHyperbolicInverse => "tanh\u{05BF}\u{00B9}(".to_string(),
            SingleByteToken::If => "If".to_string(),
            SingleByteToken::Then => "Then".to_string(),
            SingleByteToken::Else => "Else".to_string(),
            SingleByteToken::While => "While".to_string(),
            SingleByteToken::Repeat => "Repeat".to_string(),
            SingleByteToken::For => "For".to_string(),
            SingleByteToken::End => "End".to_string(),
            SingleByteToken::Return => "Return".to_string(),
            SingleByteToken::Lbl => "Lbl".to_string(),
            SingleByteToken::Goto => "Goto".to_string(),
            SingleByteToken::Pause => "Pause".to_string(),
            SingleByteToken::Stop => "Stop".to_string(),
            SingleByteToken::IncrementSkipIfGreaterThan => "IS>".to_string(),
            SingleByteToken::DecrementSkipIfLessThan => "DS<".to_string(),
            SingleByteToken::Input => "Input ".to_string(),
            SingleByteToken::Prompt => "Prompt ".to_string(),
            SingleByteToken::Disp => "Disp ".to_string(),
            SingleByteToken::DispGraph => "DispGraph".to_string(),
            SingleByteToken::Output => "Output(".to_string(),
            SingleByteToken::ClrHome => "ClrHome".to_string(),
            SingleByteToken::Fill => "Fill(".to_string(),
            SingleByteToken::SortA => "SortA(".to_string(),
            SingleByteToken::SortD => "SortD(".to_string(),
            SingleByteToken::DispTable => "DispTable".to_string(),
            SingleByteToken::Menu => "Menu(".to_string(),
            SingleByteToken::Send => "Send(".to_string(),
            SingleByteToken::Get => "Get(".to_string(),
            SingleByteToken::PlotsOn => "PlotsOn".to_string(),
            SingleByteToken::PlotsOff => "PlotsOff".to_string(),
            SingleByteToken::Angle => "Angle".to_string(),
            SingleByteToken::Plot1 => "Plot1".to_string(),
            SingleByteToken::Plot2 => "Plot2".to_string(),
            SingleByteToken::Plot3 => "Plot3".to_string(),
            SingleByteToken::ExponentMark => "^".to_string(),
            SingleByteToken::CustomXSuperscriptRadical => "X\u{221A}(".to_string(),
            SingleByteToken::OneVarStats => "1-Var Stats".to_string(),
            SingleByteToken::TwoVarStats => "2-Var Stats".to_string(),
            SingleByteToken::LinRegABPlusX => "LinReg(a+bx)".to_string(),
            SingleByteToken::ExpReg => "ExpReg".to_string(),
            SingleByteToken::LnReg => "LnReg".to_string(),
            SingleByteToken::PwrReg => "PwrReg".to_string(),
            SingleByteToken::MedMed => "Med-Med".to_string(),
            SingleByteToken::QuadReg => "QuadReg".to_string(),
            SingleByteToken::ClrList => "ClrList".to_string(),
            SingleByteToken::ClrTable => "ClrTable".to_string(),
            SingleByteToken::Histogram => "Histogram".to_string(),
            SingleByteToken::XYLine => "XYLine".to_string(),
            SingleByteToken::Scatter => "Scatter".to_string(),
            SingleByteToken::LinRegAXPlusB => "LinReg(ax+b)".to_string(),
        }
    }
}

pub struct SingleByteTokens {
    tokens: HashMap<u8, SingleByteToken>,
}

impl SingleByteTokens {
    pub fn new() -> SingleByteTokens {
        let mut single_byte_tokens: HashMap<u8, SingleByteToken> = HashMap::new();

        single_byte_tokens.insert(0x00, SingleByteToken::UnusedCodePoint);
        single_byte_tokens.insert(0x0A, SingleByteToken::R);
        single_byte_tokens.insert(0x0B, SingleByteToken::DegreeSymbol);
        single_byte_tokens.insert(0x0C, SingleByteToken::SuperscriptOne);
        single_byte_tokens.insert(0x0D, SingleByteToken::SuperscriptTwo);
        single_byte_tokens.insert(0x0E, SingleByteToken::T);
        single_byte_tokens.insert(0x0F, SingleByteToken::SuperscriptThree);
        single_byte_tokens.insert(0x01, SingleByteToken::RightArrowDMS);
        single_byte_tokens.insert(0x10, SingleByteToken::OpenParenthesis);
        single_byte_tokens.insert(0x11, SingleByteToken::CloseParenthesis);
        single_byte_tokens.insert(0x12, SingleByteToken::Round);
        single_byte_tokens.insert(0x13, SingleByteToken::PxlTest);
        single_byte_tokens.insert(0x14, SingleByteToken::Augment);
        single_byte_tokens.insert(0x15, SingleByteToken::RowSwap);
        single_byte_tokens.insert(0x16, SingleByteToken::RowPlus);
        single_byte_tokens.insert(0x17, SingleByteToken::MultiplyRow);
        single_byte_tokens.insert(0x18, SingleByteToken::MultiplyRowPlus);
        single_byte_tokens.insert(0x19, SingleByteToken::Max);
        single_byte_tokens.insert(0x1A, SingleByteToken::Min);
        single_byte_tokens.insert(0x1B, SingleByteToken::RPr);
        single_byte_tokens.insert(0x1C, SingleByteToken::RPTheta);
        single_byte_tokens.insert(0x1D, SingleByteToken::PRx);
        single_byte_tokens.insert(0x1E, SingleByteToken::PRy);
        single_byte_tokens.insert(0x1F, SingleByteToken::Median);
        single_byte_tokens.insert(0x02, SingleByteToken::RightArrowDec);
        single_byte_tokens.insert(0x20, SingleByteToken::RandM);
        single_byte_tokens.insert(0x21, SingleByteToken::Mean);
        single_byte_tokens.insert(0x22, SingleByteToken::Solve);
        single_byte_tokens.insert(0x23, SingleByteToken::Sequence);
        single_byte_tokens.insert(0x24, SingleByteToken::FnInt);
        single_byte_tokens.insert(0x25, SingleByteToken::NDeriv);
        single_byte_tokens.insert(0x26, SingleByteToken::UnusedCodePoint);
        single_byte_tokens.insert(0x27, SingleByteToken::FMin);
        single_byte_tokens.insert(0x28, SingleByteToken::FMax);
        single_byte_tokens.insert(0x29, SingleByteToken::Space);
        single_byte_tokens.insert(0x2A, SingleByteToken::QuotationMark);
        single_byte_tokens.insert(0x2B, SingleByteToken::Comma);
        single_byte_tokens.insert(0x2C, SingleByteToken::I);
        single_byte_tokens.insert(0x2D, SingleByteToken::ExclamationMark);
        single_byte_tokens.insert(0x2E, SingleByteToken::CubicReg);
        single_byte_tokens.insert(0x2F, SingleByteToken::QuartReg);
        single_byte_tokens.insert(0x03, SingleByteToken::RightArrowFrac);
        single_byte_tokens.insert(0x30, SingleByteToken::Zero);
        single_byte_tokens.insert(0x31, SingleByteToken::One);
        single_byte_tokens.insert(0x32, SingleByteToken::Two);
        single_byte_tokens.insert(0x33, SingleByteToken::Three);
        single_byte_tokens.insert(0x34, SingleByteToken::Four);
        single_byte_tokens.insert(0x35, SingleByteToken::Five);
        single_byte_tokens.insert(0x36, SingleByteToken::Six);
        single_byte_tokens.insert(0x37, SingleByteToken::Seven);
        single_byte_tokens.insert(0x38, SingleByteToken::Eight);
        single_byte_tokens.insert(0x39, SingleByteToken::Nine);
        single_byte_tokens.insert(0x3A, SingleByteToken::Period);
        single_byte_tokens.insert(0x3B, SingleByteToken::E);
        single_byte_tokens.insert(0x3C, SingleByteToken::Or);
        single_byte_tokens.insert(0x3D, SingleByteToken::Xor);
        single_byte_tokens.insert(0x3E, SingleByteToken::Colon);
        single_byte_tokens.insert(0x3F, SingleByteToken::Newline);
        single_byte_tokens.insert(0x04, SingleByteToken::RightArrow);
        single_byte_tokens.insert(0x40, SingleByteToken::And);
        single_byte_tokens.insert(0x41, SingleByteToken::LetterA);
        single_byte_tokens.insert(0x42, SingleByteToken::LetterB);
        single_byte_tokens.insert(0x43, SingleByteToken::LetterC);
        single_byte_tokens.insert(0x44, SingleByteToken::LetterD);
        single_byte_tokens.insert(0x45, SingleByteToken::LetterE);
        single_byte_tokens.insert(0x46, SingleByteToken::LetterF);
        single_byte_tokens.insert(0x47, SingleByteToken::LetterG);
        single_byte_tokens.insert(0x48, SingleByteToken::LetterH);
        single_byte_tokens.insert(0x49, SingleByteToken::LetterI);
        single_byte_tokens.insert(0x4A, SingleByteToken::LetterJ);
        single_byte_tokens.insert(0x4B, SingleByteToken::LetterK);
        single_byte_tokens.insert(0x4C, SingleByteToken::LetterL);
        single_byte_tokens.insert(0x4D, SingleByteToken::LetterM);
        single_byte_tokens.insert(0x4E, SingleByteToken::LetterN);
        single_byte_tokens.insert(0x4F, SingleByteToken::LetterO);
        single_byte_tokens.insert(0x05, SingleByteToken::Boxplot);
        single_byte_tokens.insert(0x50, SingleByteToken::LetterP);
        single_byte_tokens.insert(0x51, SingleByteToken::LetterQ);
        single_byte_tokens.insert(0x52, SingleByteToken::LetterR);
        single_byte_tokens.insert(0x53, SingleByteToken::LetterS);
        single_byte_tokens.insert(0x54, SingleByteToken::LetterT);
        single_byte_tokens.insert(0x55, SingleByteToken::LetterU);
        single_byte_tokens.insert(0x56, SingleByteToken::LetterV);
        single_byte_tokens.insert(0x57, SingleByteToken::LetterW);
        single_byte_tokens.insert(0x58, SingleByteToken::LetterX);
        single_byte_tokens.insert(0x59, SingleByteToken::LetterY);
        single_byte_tokens.insert(0x5A, SingleByteToken::LetterZ);
        single_byte_tokens.insert(0x5B, SingleByteToken::Theta);
        single_byte_tokens.insert(0x5C, SingleByteToken::Unknown2ByteCode);
        single_byte_tokens.insert(0x5D, SingleByteToken::Unknown2ByteCode);
        single_byte_tokens.insert(0x5E, SingleByteToken::Unknown2ByteCode);
        single_byte_tokens.insert(0x5F, SingleByteToken::Prgm);
        single_byte_tokens.insert(0x06, SingleByteToken::OpenBracket);
        single_byte_tokens.insert(0x60, SingleByteToken::Unknown2ByteCode);
        single_byte_tokens.insert(0x61, SingleByteToken::Unknown2ByteCode);
        single_byte_tokens.insert(0x62, SingleByteToken::Unknown2ByteCode);
        single_byte_tokens.insert(0x63, SingleByteToken::Unknown2ByteCode);
        single_byte_tokens.insert(0x64, SingleByteToken::Radian);
        single_byte_tokens.insert(0x65, SingleByteToken::Degree);
        single_byte_tokens.insert(0x66, SingleByteToken::Normal);
        single_byte_tokens.insert(0x67, SingleByteToken::Sci);
        single_byte_tokens.insert(0x68, SingleByteToken::Eng);
        single_byte_tokens.insert(0x69, SingleByteToken::Float);
        single_byte_tokens.insert(0x6A, SingleByteToken::Equals);
        single_byte_tokens.insert(0x6B, SingleByteToken::LessThan);
        single_byte_tokens.insert(0x6C, SingleByteToken::GreaterThan);
        single_byte_tokens.insert(0x6D, SingleByteToken::LessThanOrEqualTo);
        single_byte_tokens.insert(0x6E, SingleByteToken::GreaterThanOrEqualTo);
        single_byte_tokens.insert(0x6F, SingleByteToken::NotEqualTo);
        single_byte_tokens.insert(0x07, SingleByteToken::CloseBracket);
        single_byte_tokens.insert(0x70, SingleByteToken::Add);
        single_byte_tokens.insert(0x71, SingleByteToken::Subtract);
        single_byte_tokens.insert(0x72, SingleByteToken::Ans);
        single_byte_tokens.insert(0x73, SingleByteToken::Fix);
        single_byte_tokens.insert(0x74, SingleByteToken::Horiz);
        single_byte_tokens.insert(0x75, SingleByteToken::Full);
        single_byte_tokens.insert(0x76, SingleByteToken::Func);
        single_byte_tokens.insert(0x77, SingleByteToken::Param);
        single_byte_tokens.insert(0x78, SingleByteToken::Polar);
        single_byte_tokens.insert(0x79, SingleByteToken::Seq);
        single_byte_tokens.insert(0x7A, SingleByteToken::IndpntAuto);
        single_byte_tokens.insert(0x7B, SingleByteToken::IndpntAsk);
        single_byte_tokens.insert(0x7C, SingleByteToken::DependAuto);
        single_byte_tokens.insert(0x7D, SingleByteToken::DependAsk);
        single_byte_tokens.insert(0x7E, SingleByteToken::Unknown2ByteCode);
        single_byte_tokens.insert(0x7F, SingleByteToken::SquareMark);
        single_byte_tokens.insert(0x08, SingleByteToken::OpenBrace);
        single_byte_tokens.insert(0x80, SingleByteToken::PlusMark);
        single_byte_tokens.insert(0x81, SingleByteToken::DotMark);
        single_byte_tokens.insert(0x82, SingleByteToken::Multiply);
        single_byte_tokens.insert(0x83, SingleByteToken::Divide);
        single_byte_tokens.insert(0x84, SingleByteToken::Trace);
        single_byte_tokens.insert(0x85, SingleByteToken::ClrDraw);
        single_byte_tokens.insert(0x86, SingleByteToken::ZStandard);
        single_byte_tokens.insert(0x87, SingleByteToken::ZTrig);
        single_byte_tokens.insert(0x88, SingleByteToken::ZBox);
        single_byte_tokens.insert(0x89, SingleByteToken::ZoomIn);
        single_byte_tokens.insert(0x8A, SingleByteToken::ZoomOut);
        single_byte_tokens.insert(0x8B, SingleByteToken::ZSquare);
        single_byte_tokens.insert(0x8C, SingleByteToken::ZInteger);
        single_byte_tokens.insert(0x8D, SingleByteToken::ZPrevious);
        single_byte_tokens.insert(0x8E, SingleByteToken::ZDecimal);
        single_byte_tokens.insert(0x8F, SingleByteToken::ZoomStat);
        single_byte_tokens.insert(0x09, SingleByteToken::CloseBrace);
        single_byte_tokens.insert(0x90, SingleByteToken::ZoomRcl);
        single_byte_tokens.insert(0x91, SingleByteToken::PrintScreen);
        single_byte_tokens.insert(0x92, SingleByteToken::ZoomSto);
        single_byte_tokens.insert(0x93, SingleByteToken::Text);
        single_byte_tokens.insert(0x94, SingleByteToken::NPr);
        single_byte_tokens.insert(0x95, SingleByteToken::NCr);
        single_byte_tokens.insert(0x96, SingleByteToken::FnOn);
        single_byte_tokens.insert(0x97, SingleByteToken::FnOff);
        single_byte_tokens.insert(0x98, SingleByteToken::StorePic);
        single_byte_tokens.insert(0x99, SingleByteToken::RecallPic);
        single_byte_tokens.insert(0x9A, SingleByteToken::StoreGDB);
        single_byte_tokens.insert(0x9B, SingleByteToken::RecallGDB);
        single_byte_tokens.insert(0x9C, SingleByteToken::Line);
        single_byte_tokens.insert(0x9D, SingleByteToken::Vertical);
        single_byte_tokens.insert(0x9E, SingleByteToken::PtOn);
        single_byte_tokens.insert(0x9F, SingleByteToken::PtOff);
        single_byte_tokens.insert(0xA0, SingleByteToken::PtChange);
        single_byte_tokens.insert(0xA1, SingleByteToken::PxlOn);
        single_byte_tokens.insert(0xA2, SingleByteToken::PxlOff);
        single_byte_tokens.insert(0xA3, SingleByteToken::PxlChange);
        single_byte_tokens.insert(0xA4, SingleByteToken::Shade);
        single_byte_tokens.insert(0xA5, SingleByteToken::Circle);
        single_byte_tokens.insert(0xA6, SingleByteToken::Horizontal);
        single_byte_tokens.insert(0xA7, SingleByteToken::Tangent);
        single_byte_tokens.insert(0xA8, SingleByteToken::DrawInv);
        single_byte_tokens.insert(0xA9, SingleByteToken::DrawF);
        single_byte_tokens.insert(0xAA, SingleByteToken::Unknown2ByteCode);
        single_byte_tokens.insert(0xAB, SingleByteToken::Rand);
        single_byte_tokens.insert(0xAC, SingleByteToken::Pi);
        single_byte_tokens.insert(0xAD, SingleByteToken::GetKey);
        single_byte_tokens.insert(0xAE, SingleByteToken::Apostrophe);
        single_byte_tokens.insert(0xAF, SingleByteToken::QuestionMark);
        single_byte_tokens.insert(0xB0, SingleByteToken::Negative);
        single_byte_tokens.insert(0xB1, SingleByteToken::Int);
        single_byte_tokens.insert(0xB2, SingleByteToken::Abs);
        single_byte_tokens.insert(0xB3, SingleByteToken::Det);
        single_byte_tokens.insert(0xB4, SingleByteToken::Identity);
        single_byte_tokens.insert(0xB5, SingleByteToken::Dim);
        single_byte_tokens.insert(0xB6, SingleByteToken::Sum);
        single_byte_tokens.insert(0xB7, SingleByteToken::Prod);
        single_byte_tokens.insert(0xB8, SingleByteToken::Not);
        single_byte_tokens.insert(0xB9, SingleByteToken::IPart);
        single_byte_tokens.insert(0xBA, SingleByteToken::FPart);
        single_byte_tokens.insert(0xBB, SingleByteToken::Unknown2ByteCode);
        single_byte_tokens.insert(0xBC, SingleByteToken::SquareRoot);
        single_byte_tokens.insert(0xBD, SingleByteToken::CubeRoot);
        single_byte_tokens.insert(0xBE, SingleByteToken::NaturalLogarithm);
        single_byte_tokens.insert(0xBF, SingleByteToken::Exponential);
        single_byte_tokens.insert(0xC0, SingleByteToken::Logarithm);
        single_byte_tokens.insert(0xC1, SingleByteToken::TenToThePowerOf);
        single_byte_tokens.insert(0xC2, SingleByteToken::ShortSine);
        single_byte_tokens.insert(0xC3, SingleByteToken::ShortSineInverse);
        single_byte_tokens.insert(0xC4, SingleByteToken::ShortCosine);
        single_byte_tokens.insert(0xC5, SingleByteToken::ShortCosineInverse);
        single_byte_tokens.insert(0xC6, SingleByteToken::ShortTangent);
        single_byte_tokens.insert(0xC7, SingleByteToken::ShortTangentInverse);
        single_byte_tokens.insert(0xC8, SingleByteToken::ShortSineHyperbolic);
        single_byte_tokens.insert(0xC9, SingleByteToken::ShortSineHyperbolicInverse);
        single_byte_tokens.insert(0xCA, SingleByteToken::ShortCosineHyperbolic);
        single_byte_tokens.insert(0xCB, SingleByteToken::ShortCosineHyperbolicInverse);
        single_byte_tokens.insert(0xCC, SingleByteToken::ShortTangentHyperbolic);
        single_byte_tokens.insert(0xCD, SingleByteToken::ShortTangentHyperbolicInverse);
        single_byte_tokens.insert(0xCE, SingleByteToken::If);
        single_byte_tokens.insert(0xCF, SingleByteToken::Then);
        single_byte_tokens.insert(0xD0, SingleByteToken::Else);
        single_byte_tokens.insert(0xD1, SingleByteToken::While);
        single_byte_tokens.insert(0xD2, SingleByteToken::Repeat);
        single_byte_tokens.insert(0xD3, SingleByteToken::For);
        single_byte_tokens.insert(0xD4, SingleByteToken::End);
        single_byte_tokens.insert(0xD5, SingleByteToken::Return);
        single_byte_tokens.insert(0xD6, SingleByteToken::Lbl);
        single_byte_tokens.insert(0xD7, SingleByteToken::Goto);
        single_byte_tokens.insert(0xD8, SingleByteToken::Pause);
        single_byte_tokens.insert(0xD9, SingleByteToken::Stop);
        single_byte_tokens.insert(0xDA, SingleByteToken::IncrementSkipIfGreaterThan);
        single_byte_tokens.insert(0xDB, SingleByteToken::DecrementSkipIfLessThan);
        single_byte_tokens.insert(0xDC, SingleByteToken::Input);
        single_byte_tokens.insert(0xDD, SingleByteToken::Prompt);
        single_byte_tokens.insert(0xDE, SingleByteToken::Disp);
        single_byte_tokens.insert(0xDF, SingleByteToken::DispGraph);
        single_byte_tokens.insert(0xE0, SingleByteToken::Output);
        single_byte_tokens.insert(0xE1, SingleByteToken::ClrHome);
        single_byte_tokens.insert(0xE2, SingleByteToken::Fill);
        single_byte_tokens.insert(0xE3, SingleByteToken::SortA);
        single_byte_tokens.insert(0xE4, SingleByteToken::SortD);
        single_byte_tokens.insert(0xE5, SingleByteToken::DispTable);
        single_byte_tokens.insert(0xE6, SingleByteToken::Menu);
        single_byte_tokens.insert(0xE7, SingleByteToken::Send);
        single_byte_tokens.insert(0xE8, SingleByteToken::Get);
        single_byte_tokens.insert(0xE9, SingleByteToken::PlotsOn);
        single_byte_tokens.insert(0xEA, SingleByteToken::PlotsOff);
        single_byte_tokens.insert(0xEB, SingleByteToken::Angle);
        single_byte_tokens.insert(0xEC, SingleByteToken::Plot1);
        single_byte_tokens.insert(0xED, SingleByteToken::Plot2);
        single_byte_tokens.insert(0xEE, SingleByteToken::Plot3);
        single_byte_tokens.insert(0xEF, SingleByteToken::Unknown2ByteCode);
        single_byte_tokens.insert(0xF0, SingleByteToken::ExponentMark);
        single_byte_tokens.insert(0xF1, SingleByteToken::CustomXSuperscriptRadical);
        single_byte_tokens.insert(0xF2, SingleByteToken::OneVarStats);
        single_byte_tokens.insert(0xF3, SingleByteToken::TwoVarStats);
        single_byte_tokens.insert(0xF4, SingleByteToken::LinRegABPlusX);
        single_byte_tokens.insert(0xF5, SingleByteToken::ExpReg);
        single_byte_tokens.insert(0xF6, SingleByteToken::LnReg);
        single_byte_tokens.insert(0xF7, SingleByteToken::PwrReg);
        single_byte_tokens.insert(0xF8, SingleByteToken::MedMed);
        single_byte_tokens.insert(0xF9, SingleByteToken::QuadReg);
        single_byte_tokens.insert(0xFA, SingleByteToken::ClrList);
        single_byte_tokens.insert(0xFB, SingleByteToken::ClrTable);
        single_byte_tokens.insert(0xFC, SingleByteToken::Histogram);
        single_byte_tokens.insert(0xFD, SingleByteToken::XYLine);
        single_byte_tokens.insert(0xFE, SingleByteToken::Scatter);
        single_byte_tokens.insert(0xFF, SingleByteToken::LinRegAXPlusB);

        SingleByteTokens {
            tokens: single_byte_tokens,
        }
    }

    pub fn get_token(&self, byte: u8) -> Result<String, String> {
        let token_map = &self.tokens;
        let key_value = token_map.get_key_value(&byte);

        match key_value {
            Some((_, value)) => Ok(value.to_string()),
            None => Err(format!("No token is associated with {}", byte)),
        }
    }
}
