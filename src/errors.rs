use std::fmt::Debug;

pub enum CliError {
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
}

impl Debug for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl CliError {
    fn to_string(&self) -> String {
        match self {
            CliError::FileRead(err) => format!("Failed to read file: {}", err),
            CliError::FileWrite(err) => format!("Failed to write file: {}", err),
            CliError::Match(value, enum_name) => {
                format!("Failed to match {} in {}", value, enum_name)
            }
            CliError::Json(err) => format!("Failed to parse json: {}", err),
            CliError::InvalidExtension(extension) => {
                format!(
                    "Invalid file extension {}, only 8xp, 83p, 82p, and txt are supported",
                    extension
                )
            }
            CliError::MissingExtension => "File extension is missing".to_string(),
            CliError::TokenNotFound(token) => format!("Token not found: {}", token),
            CliError::InvalidByteLength(bytes) => format!("Invalid byte length: {}", bytes),
            CliError::MissingProgramInfo(info) => format!("Missing program info: {}", info),
            CliError::InvalidNameLength => "Name must be 8 or less characters".to_string(),
            CliError::InvalidNameCharacters => "Name must be alphabetic".to_string(),
            CliError::FailedToFindFile(file) => format!("Failed to find file: {}", file),
            CliError::IncompatibleFileType(provided, accepted) => {
                format!(
                    "Incompatible file type: {}, accepted: {}",
                    provided, accepted
                )
            }
            CliError::FailedToDeleteFile(err) => format!("Failed to delete file: {}", err),
            CliError::Quit(reason) => format!("Quit: {}", reason),
            CliError::FailedToWriteFile(file, err) => {
                format!("Failed to write file {}: {}", file, err)
            }
            CliError::InvalidCommentLength => "Comment must be 42 characters or less".to_string(),
            CliError::MassConversionInputNotDirectory(conversion_type) => {
                format!(
                    "When mass {} the input path must lead to a directory",
                    conversion_type
                )
            }
            CliError::MassConversionOutputNotDirectory(conversion_type) => {
                format!(
                    "When mass {} the output path must lead to a directory",
                    conversion_type
                )
            }
            CliError::FailedToReadDirectory(err) => format!("Failed to read directory: {}", err),
        }
    }

    pub fn print(self) -> CliError {
        eprintln!("{}", self.to_string());
        self
    }

    pub fn exit(&self) -> ! {
        match self {
            CliError::Quit(_) => std::process::exit(0),
            _ => std::process::exit(1),
        }
    }
}
