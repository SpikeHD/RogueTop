use serde::{Deserialize, Serialize};
use std::fs;

use crate::util::paths::get_mods_path;

#[derive(Default, Serialize, Deserialize)]
pub struct ModConfig {
  pub disabled: Vec<String>,
  pub load_order: Vec<String>,
}

pub mod replacer;

#[tauri::command]
pub fn open_mods_folder() {
  let mods_path = get_mods_path();
  open::that(mods_path).unwrap_or_default();
}

/// Get mod config
pub fn get_mod_config() -> Result<ModConfig, std::io::Error> {
  let mods_path = get_mods_path();
  let default_config = ModConfig {
    disabled: vec![],
    load_order: vec![],
  };
  let config_path = mods_path.join("mods.json");

  if let Ok(data) = fs::read(&config_path) {
    let config = serde_json::from_slice(&data)?;
    return Ok(config);
  } else {
    // Create config
    let config_str = serde_json::to_string(&default_config)?;

    fs::write(&config_path, config_str)?;
  }

  Ok(default_config)
}
