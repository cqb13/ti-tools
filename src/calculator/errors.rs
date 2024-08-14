use std::fmt::Debug;

pub enum TiToolsError {
    FileRead(String),
    FileWrite(String),
    /**Value, Enum Name */
    Match(String, String),
    Json(String),
    InvalidExtension(String),
    MissingExtension,
    TokenNotFound(String),
    InvalidByteLength(String),
    MissingProgramInfo(String),
    InvalidNameLength,
    InvalidNameCharacters,
    FailedToFindFile(String),
    /**Provided, Accepted */
    IncompatibleFileType(String, String),
    FailedToDeleteFile(String),
    Quit(String),
    /**File, Error */
    FailedToWriteFile(String, String),
    InvalidCommentLength,
    /**Type encode|decode */
    MassConversionInputNotDirectory(String),
    /**Type encode|decode */
    MassConversionOutputNotDirectory(String),
    FailedToReadDirectory(String),
    FailedToSerializeJson(String),
    FailedToReadFile(String),
    FailedToDeserializeJson(String),
}

impl Debug for TiToolsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl TiToolsError {
    fn to_string(&self) -> String {
        match self {
            TiToolsError::FileRead(err) => format!("Failed to read file: {}", err),
            TiToolsError::FileWrite(err) => format!("Failed to write file: {}", err),
            TiToolsError::Match(value, enum_name) => {
                format!("Failed to match {} in {}", value, enum_name)
            }
            TiToolsError::Json(err) => format!("Failed to parse json: {}", err),
            TiToolsError::InvalidExtension(extension) => {
                format!(
                    "Invalid file extension {}, only 8xp, 83p, 82p, json, and txt are supported",
                    extension
                )
            }
            TiToolsError::MissingExtension => "File extension is missing".to_string(),
            TiToolsError::TokenNotFound(token) => format!("Token not found: {}", token),
            TiToolsError::InvalidByteLength(bytes) => format!("Invalid byte length: {}", bytes),
            TiToolsError::MissingProgramInfo(info) => format!("Missing program info: {}", info),
            TiToolsError::InvalidNameLength => "Name must be 8 or less characters".to_string(),
            TiToolsError::InvalidNameCharacters => "Name must be alphabetic".to_string(),
            TiToolsError::FailedToFindFile(file) => format!("Failed to find file: {}", file),
            TiToolsError::IncompatibleFileType(provided, accepted) => {
                format!(
                    "Incompatible file type: {}, accepted: {}",
                    provided, accepted
                )
            }
            TiToolsError::FailedToDeleteFile(err) => format!("Failed to delete file: {}", err),
            TiToolsError::Quit(reason) => format!("Quit: {}", reason),
            TiToolsError::FailedToWriteFile(file, err) => {
                format!("Failed to write file {}: {}", file, err)
            }
            TiToolsError::InvalidCommentLength => {
                "Comment must be 42 characters or less".to_string()
            }
            TiToolsError::MassConversionInputNotDirectory(conversion_type) => {
                format!(
                    "When mass {} the input path must lead to a directory",
                    conversion_type
                )
            }
            TiToolsError::MassConversionOutputNotDirectory(conversion_type) => {
                format!(
                    "When mass {} the output path must lead to a directory",
                    conversion_type
                )
            }
            TiToolsError::FailedToReadDirectory(err) => {
                format!("Failed to read directory: {}", err)
            }
            TiToolsError::FailedToSerializeJson(err) => {
                format!("Failed to serialize json: {}", err)
            }
            TiToolsError::FailedToReadFile(err) => format!("Failed to read file: {}", err),
            TiToolsError::FailedToDeserializeJson(err) => {
                format!("Failed to deserialize json: {}", err)
            }
        }
    }

    pub fn print(self) -> TiToolsError {
        eprintln!("{}", self.to_string());
        self
    }

    pub fn exit(&self) -> ! {
        match self {
            TiToolsError::Quit(_) => std::process::exit(0),
            _ => std::process::exit(1),
        }
    }
}
