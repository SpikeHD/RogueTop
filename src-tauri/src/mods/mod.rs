use serde::{Deserialize, Serialize};
use std::fs;

use crate::util::paths::get_mods_path;

#[derive(Default, Serialize, Deserialize)]
pub struct ModConfig {
  pub disabled: Vec<String>,
  pub load_order: Vec<String>,
}

pub mod plugin;
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

#[tauri::command]
pub fn get_mods_list() -> Result<Vec<String>, tauri::Error> {
  // get list of all replacers AND plugins
  let replacers = replacer::get_replacer_list();
  let plugins = plugin::get_plugin_list();
  let mut mod_config = get_mod_config()?;

  // Merge
  let mut mods = vec![];

  for mod_name in replacers {
    mods.push(mod_name.clone());

    // If this mod didn't exist in the config, add it to the load order
    if !mod_config.load_order.contains(&mod_name) {
      mod_config.load_order.push(mod_name);
    }
  }

  for mod_name in plugins {
    mods.push(mod_name.clone());

    // If this mod didn't exist in the config, add it to the load order
    if !mod_config.load_order.contains(&mod_name) {
      mod_config.load_order.push(mod_name);
    }
  }

  // Also write to global mods.json
  let mods_folder = get_mods_path();
  let mods_json = mods_folder.join("mods.json");

  let mods_str = serde_json::to_string(&mod_config)?;

  fs::write(mods_json, mods_str)?;

  Ok(mods)
}
