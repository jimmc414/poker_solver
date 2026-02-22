use serde::Serialize;

/// Application-level error type.
/// Serializable for IPC transport to the frontend.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Evaluation error: {0}")]
    Eval(String),

    #[error("Range error: {0}")]
    Range(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),
}

/// Tauri requires `Serialize` on command return errors.
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
