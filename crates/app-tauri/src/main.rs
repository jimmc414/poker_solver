// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod error;
mod setup;
mod state;

use state::AppState;

fn main() {
    let data_dir = setup::ensure_data_dir().expect("Failed to create data directory");
    let app_state = AppState::new(data_dir);

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::eval::evaluate_hand,
            commands::eval::equity_calculation,
            commands::range::compute_equity,
            commands::range::load_preset,
            commands::range::save_preset,
            commands::settings::get_config,
            commands::settings::update_config,
            commands::settings::get_data_dir,
        ])
        .run(tauri::generate_context!())
        .expect("Error running tauri application");
}
