mod create;
mod decode;
mod encode;
pub mod program;

pub enum EncodeMode {
    Min,
    Max,
    Smart,
}

impl EncodeMode {
    pub fn from_string(encode_mode: &str) -> Result<EncodeMode, String> {
        match encode_mode {
            "min" => Ok(EncodeMode::Min),
            "max" => Ok(EncodeMode::Max),
            "smart" => Ok(EncodeMode::Smart),
            _ => Err(format!("Unknown encode mode: {}", encode_mode)),
        }
    }
}

pub enum DisplayMode {
    Pretty,
    Accessible,
    TiAscii,
}

impl DisplayMode {
    pub fn from_string(display_mode: &str) -> Result<DisplayMode, String> {
        match display_mode {
            "pretty" => Ok(DisplayMode::Pretty),
            "accessible" => Ok(DisplayMode::Accessible),
            "ti" => Ok(DisplayMode::TiAscii),
            _ => Err(format!("Unknown display mode: {}", display_mode)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DisplayMode::Pretty => "pretty",
            DisplayMode::Accessible => "accessible",
            DisplayMode::TiAscii => "ti",
        }
        .to_string()
    }
}

#[derive(Debug, Eq)]
pub enum Model {
    TI82,
    TI83,
    TI82ST,
    TI82STFR,
    TI76FR,
    TI83Plus,
    TI83PlusSE,
    TI83PlusFR,
    TI82Plus,
    TI84Plus,
    TI84PlusSE,
    TI83PlusFRUSB,
    TI84PFR,
    TI84PlusPSE,
    TI82A,
    TI84PlusT,
    TI84PlusCSE,
    TI84PlusCE,
    TI84PlusCET,
    TI83PCE,
    TI83PCEEP,
    TI84PlusCEPY,
    TI84PlusCETPE,
    TI82AEP,
    Latest,
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Model {
    pub fn from_string(model: &str) -> Result<Model, String> {
        match model {
            "TI-82" => Ok(Model::TI82),
            "TI-83" => Ok(Model::TI83),
            "TI-82ST" => Ok(Model::TI82ST),
            "TI-82ST.fr" => Ok(Model::TI82STFR),
            "TI-76.fr" => Ok(Model::TI76FR),
            "TI-83+" => Ok(Model::TI83Plus),
            "TI-83+SE" => Ok(Model::TI83PlusSE),
            "TI-83+.fr" => Ok(Model::TI83PlusFR),
            "TI-82+" => Ok(Model::TI82Plus),
            "TI-84+" => Ok(Model::TI84Plus),
            "TI-84+SE" => Ok(Model::TI84PlusSE),
            "TI-83+.fr:USB" => Ok(Model::TI83PlusFRUSB),
            "TI-84P.fr" => Ok(Model::TI84PFR),
            "TI-84+PSE" => Ok(Model::TI84PlusPSE),
            "TI-82A" => Ok(Model::TI82A),
            "TI-84+T" => Ok(Model::TI84PlusT),
            "TI-84+CSE" => Ok(Model::TI84PlusCSE),
            "TI-84+CE" => Ok(Model::TI84PlusCE),
            "TI-84+CET" => Ok(Model::TI84PlusCET),
            "TI-83PCE" => Ok(Model::TI83PCE),
            "TI-83PCEEP" => Ok(Model::TI83PCEEP),
            "TI-84+CEPY" => Ok(Model::TI84PlusCEPY),
            "TI-84+CETPE" => Ok(Model::TI84PlusCETPE),
            "TI-82AEP" => Ok(Model::TI82AEP),
            "latest" => Ok(Model::Latest),
            _ => Err(format!("Unknown model: {}", model)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Model::TI82 => "TI-82",
            Model::TI83 => "TI-83",
            Model::TI82ST => "TI-82ST",
            Model::TI82STFR => "TI-82ST.fr",
            Model::TI76FR => "TI-76.fr",
            Model::TI83Plus => "TI-83+",
            Model::TI83PlusSE => "TI-83+SE",
            Model::TI83PlusFR => "TI-83+.fr",
            Model::TI82Plus => "TI-82+",
            Model::TI84Plus => "TI-84+",
            Model::TI84PlusSE => "TI-84+SE",
            Model::TI83PlusFRUSB => "TI-83+.fr:USB",
            Model::TI84PFR => "TI-84P.fr",
            Model::TI84PlusPSE => "TI-84+PSE",
            Model::TI82A => "TI-82A",
            Model::TI84PlusT => "TI-84+T",
            Model::TI84PlusCSE => "TI-84+CSE",
            Model::TI84PlusCE => "TI-84+CE",
            Model::TI84PlusCET => "TI-84+CET",
            Model::TI83PCE => "TI-83PCE",
            Model::TI83PCEEP => "TI-83PCEEP",
            Model::TI84PlusCEPY => "TI-84+CEPY",
            Model::TI84PlusCETPE => "TI-84+CETPE",
            Model::TI82AEP => "TI-82AEP",
            Model::Latest => "latest",
        }
        .to_string()
    }

    pub fn model_order(&self) -> u32 {
        match self {
            Model::TI82 => 10,
            Model::TI83 | Model::TI82ST | Model::TI82STFR | Model::TI76FR => 20,
            Model::TI83Plus | Model::TI83PlusSE | Model::TI83PlusFR | Model::TI82Plus => 30,
            Model::TI84Plus
            | Model::TI84PlusSE
            | Model::TI83PlusFRUSB
            | Model::TI84PFR
            | Model::TI84PlusPSE => 40,
            Model::TI82A | Model::TI84PlusT => 45,
            Model::TI84PlusCSE => 50,
            Model::TI84PlusCE
            | Model::TI84PlusCET
            | Model::TI83PCE
            | Model::TI83PCEEP
            | Model::TI84PlusCEPY
            | Model::TI84PlusCETPE
            | Model::TI82AEP => 60,
            Model::Latest => 9999999,
        }
    }

    pub fn display_models() {
        println!(
            "TI-82\n\
            TI-83\n\
            TI-82ST\n\
            TI-82ST.fr\n\
            TI-76.fr\n\
            TI-83+\n\
            TI-83+SE\n\
            TI-83+.fr\n\
            TI-82+\n\
            TI-84+\n\
            TI-84+SE\n\
            TI-83+.fr:USB\n\
            TI-84P.fr\n\
            TI-84+PSE\n\
            TI-82A\n\
            TI-84+T\n\
            TI-84+CSE\n\
            TI-84+CE\n\
            TI-84+CET\n\
            TI-83PCE\n\
            TI-83PCEEP\n\
            TI-84+CEPY\n\
            TI-84+CETPE\n\
            TI-82AEP\n\
            latest"
        );
    }
}
