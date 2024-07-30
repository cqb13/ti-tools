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
}

impl CliError {
    pub fn to_string(&self) -> String {
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
        }
    }
}
