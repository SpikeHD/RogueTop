use std::{fs, path::PathBuf};

use crate::util::paths::get_mods_path;
use super::get_mod_config;

/// Get a file
pub fn get_alt_file(path: PathBuf) -> Result<Vec<u8>, std::io::Error> {
  // Path passed in should be relative to the mods/textures directory
  let mods_path = get_mods_path();

  // For each mod in the load order, check if the file exists
  for mod_name in get_replacer_list() {
    let mod_path = mods_path.join(&mod_name).join(&path);

    if fs::metadata(&mod_path).is_ok() {
      return fs::read(&mod_path);
    }
  }

  Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
}

/// Get mod list
#[tauri::command]
pub fn get_replacer_list() -> Vec<String> {
  let mods_path = get_mods_path();
  let mut mod_config = get_mod_config().unwrap_or_default();

  let mut mods = vec![];

  if let Ok(entries) = fs::read_dir(mods_path) {
    for entry in entries {
      if let Ok(entry) = entry {
        // Check if dir, continue if not
        if !entry.file_type().unwrap().is_dir() {
          continue;
        }

        mods.push(entry.file_name().to_string_lossy().to_string());

        // If this mod didn't exist in the config, add it to the load order
        if !mod_config.load_order.contains(&entry.file_name().to_string_lossy().to_string()) {
          mod_config.load_order.push(entry.file_name().to_string_lossy().to_string());
        }
      }
    }
  }

  // Write the config back out
  let config_str = serde_json::to_string(&mod_config).expect("Failed to serialize config!");
  fs::write(get_mods_path().join("mods.json"), config_str).expect("Failed to write config!");

  mods
}
