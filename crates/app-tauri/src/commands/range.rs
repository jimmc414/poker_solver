use serde::{Deserialize, Serialize};
use tauri::State;

use crate::error::AppError;
use crate::state::AppState;

#[derive(Serialize)]
pub struct RangeEquityResult {
    pub equity: f64,
}

/// Compute equity for a range vs range matchup (placeholder — full implementation in M2).
#[tauri::command]
pub fn compute_equity(
    range1: Vec<f64>,
    range2: Vec<f64>,
    _board: Vec<u8>,
    _state: State<'_, AppState>,
) -> Result<RangeEquityResult, AppError> {
    if range1.len() != 169 || range2.len() != 169 {
        return Err(AppError::Range(
            "Range arrays must have exactly 169 elements".into(),
        ));
    }
    // Placeholder: return 50% equity until full range-vs-range enumeration in M2
    Ok(RangeEquityResult { equity: 0.5 })
}

#[derive(Serialize)]
pub struct PresetData {
    pub name: String,
    pub weights: Vec<f64>,
}

/// Load a preset range by name.
#[tauri::command]
pub fn load_preset(name: String) -> Result<PresetData, AppError> {
    // Built-in presets
    let weights = match name.as_str() {
        "utg-open" => utg_open_weights(),
        "btn-open" => btn_open_weights(),
        "bb-defend" => bb_defend_weights(),
        _ => return Err(AppError::Range(format!("Unknown preset: {name}"))),
    };
    Ok(PresetData { name, weights })
}

#[derive(Deserialize)]
pub struct SavePresetArgs {
    pub name: String,
    pub weights: Vec<f64>,
}

/// Save a custom preset (writes to data dir).
#[tauri::command]
pub fn save_preset(args: SavePresetArgs, state: State<'_, AppState>) -> Result<(), AppError> {
    let path = state.data_dir.join("presets");
    std::fs::create_dir_all(&path)?;
    let file = path.join(format!("{}.json", args.name));
    let json = serde_json::to_string_pretty(&args.weights)?;
    std::fs::write(file, json)?;
    Ok(())
}

// Simplified placeholder presets (full preset data in M2)
fn utg_open_weights() -> Vec<f64> {
    // ~15% range: top pairs, broadway, suited connectors
    let mut w = vec![0.0; 169];
    // Pocket pairs 77+
    for i in [0, 14, 28, 42, 56, 70, 84] {
        w[i] = 100.0;
    }
    w
}

fn btn_open_weights() -> Vec<f64> {
    // ~45% range: wider opening
    let mut w = vec![0.0; 169];
    for slot in w.iter_mut().take(80) {
        *slot = 100.0;
    }
    w
}

fn bb_defend_weights() -> Vec<f64> {
    // ~35% range: moderate defense
    let mut w = vec![0.0; 169];
    for slot in w.iter_mut().take(60) {
        *slot = 100.0;
    }
    w
}
