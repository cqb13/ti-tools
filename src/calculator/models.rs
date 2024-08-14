use crate::calculator::errors::TiToolsError;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Eq, Serialize, Deserialize)]
pub struct ModelDetails {
    pub model: Model,
    pub signature: String,
    pub product_id: u8,
    pub language: String,
}

impl ModelDetails {
    pub fn new(model: Model, signature: &str, product_id: u8, language: &str) -> ModelDetails {
        ModelDetails {
            model,
            signature: signature.to_string(),
            product_id,
            language: language.to_string(),
        }
    }

    pub fn from_byte(byte: u8, signature: &str) -> Result<ModelDetails, TiToolsError> {
        match byte {
            0x00 => match signature {
                "**TI83F*" => Ok(ModelDetails::new(Model::Latest, "**TI83F*", 0x00, "en")),
                _ => Ok(ModelDetails::new(Model::TI82, "**TI82**", 0x00, "en")),
            },
            0x04 => Ok(ModelDetails::new(Model::TI83Plus, "**TI83F*", 0x04, "en")),
            0x0A => Ok(ModelDetails::new(Model::TI84Plus, "**TI83F*", 0x0A, "en")),
            0x0B => Ok(ModelDetails::new(Model::TI82A, "**TI83F*", 0x0B, "fr")),
            0x0F => Ok(ModelDetails::new(
                Model::TI84PlusCSE,
                "**TI83F*",
                0x0F,
                "en",
            )),
            0x13 => Ok(ModelDetails::new(Model::TI84PlusCE, "**TI83F*", 0x13, "en")),
            0x1B => Ok(ModelDetails::new(Model::TI84PlusT, "**TI83F*", 0x1B, "en")),
            _ => Err(TiToolsError::Match(
                format!("{}", byte),
                "Model Details".to_string(),
            )),
        }
    }

    pub fn from_model(model: &Model) -> ModelDetails {
        match model {
            Model::TI82 => ModelDetails::new(Model::TI82, "**TI82**", 0x00, "en"),
            Model::TI83 => ModelDetails::new(Model::TI83, "**TI83**", 0x00, "en"),
            Model::TI82ST => ModelDetails::new(Model::TI82ST, "**TI83**", 0x00, "en"),
            Model::TI82STFR => ModelDetails::new(Model::TI82STFR, "**TI83**", 0x00, "fr"),
            Model::TI76FR => ModelDetails::new(Model::TI76FR, "**TI83**", 0x00, "fr"),
            Model::TI83Plus => ModelDetails::new(Model::TI83Plus, "**TI83F*", 0x04, "en"),
            Model::TI83PlusSE => ModelDetails::new(Model::TI83PlusSE, "**TI83F*", 0x04, "en"),
            Model::TI83PlusFR => ModelDetails::new(Model::TI83PlusFR, "**TI83F*", 0x04, "fr"),
            Model::TI82Plus => ModelDetails::new(Model::TI82Plus, "**TI83F*", 0x04, "fr"),
            Model::TI84Plus => ModelDetails::new(Model::TI84Plus, "**TI83F*", 0x0A, "en"),
            Model::TI84PlusSE => ModelDetails::new(Model::TI84PlusSE, "**TI83F*", 0x0A, "en"),
            Model::TI83PlusFRUSB => ModelDetails::new(Model::TI83PlusFRUSB, "**TI83F*", 0x0A, "fr"),
            Model::TI84PFR => ModelDetails::new(Model::TI84PFR, "**TI83F*", 0x0A, "fr"),
            Model::TI84PlusPSE => ModelDetails::new(Model::TI84PlusPSE, "**TI83F*", 0x0A, "en"),
            Model::TI82A => ModelDetails::new(Model::TI82A, "**TI83F*", 0x0B, "fr"),
            Model::TI84PlusT => ModelDetails::new(Model::TI84PlusT, "**TI83F*", 0x1B, "en"),
            Model::TI84PlusCSE => ModelDetails::new(Model::TI84PlusCSE, "**TI83F*", 0x0F, "en"),
            Model::TI84PlusCE => ModelDetails::new(Model::TI84PlusCE, "**TI83F*", 0x13, "en"),
            Model::TI84PlusCET => ModelDetails::new(Model::TI84PlusCET, "**TI83F*", 0x13, "en"),
            Model::TI83PCE => ModelDetails::new(Model::TI83PCE, "**TI83F*", 0x13, "fr"),
            Model::TI83PCEEP => ModelDetails::new(Model::TI83PCEEP, "**TI83F*", 0x13, "fr"),
            Model::TI84PlusCEPY => ModelDetails::new(Model::TI84PlusCEPY, "**TI83F*", 0x13, "en"),
            Model::TI84PlusCETPE => ModelDetails::new(Model::TI84PlusCETPE, "**TI83F*", 0x13, "en"),
            Model::TI82AEP => ModelDetails::new(Model::TI82AEP, "**TI83F*", 0x00, "fr"),
            Model::Latest => ModelDetails::new(Model::Latest, "**TI83F*", 0x00, "en"),
        }
    }
}

impl PartialEq for ModelDetails {
    fn eq(&self, other: &Self) -> bool {
        self.model == other.model
    }
}

#[derive(Debug, Eq, Clone, Serialize)]
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
    pub fn from_string(model: &str) -> Model {
        match model {
            "TI-82" => Model::TI82,
            "TI-83" => Model::TI83,
            "TI-82ST" => Model::TI82ST,
            "TI-82ST.fr" => Model::TI82STFR,
            "TI-76.fr" => Model::TI76FR,
            "TI-83+" => Model::TI83Plus,
            "TI-83+SE" => Model::TI83PlusSE,
            "TI-83+.fr" => Model::TI83PlusFR,
            "TI-82+" => Model::TI82Plus,
            "TI-84+" => Model::TI84Plus,
            "TI-84+SE" => Model::TI84PlusSE,
            "TI-83+.fr:USB" => Model::TI83PlusFRUSB,
            "TI-84P.fr" => Model::TI84PFR,
            "TI-84+PSE" => Model::TI84PlusPSE,
            "TI-82A" => Model::TI82A,
            "TI-84+T" => Model::TI84PlusT,
            "TI-84+CSE" => Model::TI84PlusCSE,
            "TI-84+CE" => Model::TI84PlusCE,
            "TI-84+CET" => Model::TI84PlusCET,
            "TI-83PCE" => Model::TI83PCE,
            "TI-83PCEEP" => Model::TI83PCEEP,
            "TI-84+CEPY" => Model::TI84PlusCEPY,
            "TI-84+CETPE" => Model::TI84PlusCETPE,
            "TI-82AEP" => Model::TI82AEP,
            "latest" => Model::Latest,
            _ => Model::Latest,
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
