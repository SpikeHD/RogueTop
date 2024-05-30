use std::fs;

use crate::util::paths::get_plugins_path;

#[tauri::command]
pub fn get_plugin_list() -> Vec<String> {
  let mods_path = get_plugins_path();
  let mut mods = vec![];

  if let Ok(entries) = fs::read_dir(mods_path) {
    for entry in entries.flatten() {
      // Check if dir, continue if not
      if !entry.file_type().unwrap().is_dir() {
        continue;
      }

      let name = format!("plugins/{}", entry.file_name().to_string_lossy().to_string());

      mods.push(name);
    }
  }

  mods
}