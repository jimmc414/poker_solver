use serde::{Deserialize, Serialize};
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub solver_threads: usize,
    pub max_memory_mb: usize,
    pub auto_save: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            solver_threads: num_cpus(),
            max_memory_mb: 4096,
            auto_save: true,
        }
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

/// Get the current application configuration.
#[tauri::command]
pub fn get_config(state: State<'_, AppState>) -> Result<AppConfig, AppError> {
    let config_path = state.data_dir.join("config.toml");
    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(AppConfig::default())
    }
}

/// Update the application configuration.
#[tauri::command]
pub fn update_config(config: AppConfig, state: State<'_, AppState>) -> Result<(), AppError> {
    let config_path = state.data_dir.join("config.toml");
    let toml_str = toml::to_string_pretty(&config)
        .map_err(|e| AppError::Config(e.to_string()))?;
    std::fs::write(config_path, toml_str)?;
    Ok(())
}

/// Get the application data directory path.
#[tauri::command]
pub fn get_data_dir(state: State<'_, AppState>) -> Result<String, AppError> {
    Ok(state.data_dir.display().to_string())
}
