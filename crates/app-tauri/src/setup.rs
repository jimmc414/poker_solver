use std::path::PathBuf;
use tracing::info;

/// Returns the application data directory, creating it if needed.
/// Uses `~/.poker-solver/` as the canonical location.
pub fn ensure_data_dir() -> Result<PathBuf, std::io::Error> {
    let base = directories::BaseDirs::new()
        .map(|d| d.home_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    let data_dir = base.join(".poker-solver");

    let subdirs = ["solutions", "hands"];
    for sub in &subdirs {
        let dir = data_dir.join(sub);
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
            info!("Created directory: {}", dir.display());
        }
    }

    Ok(data_dir)
}
