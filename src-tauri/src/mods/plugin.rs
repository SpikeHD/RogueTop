use std::fs;

use crate::{error, success, util::paths::get_plugins_path, warn};

use super::get_mod_config;

/// Load a single plugin
pub fn load_plugin(win: &tauri::WebviewWindow, name: String) {
  let plugins_path = get_plugins_path();
  let plugin_path = plugins_path.join(&name);

  if !plugin_path.exists() {
    return;
  }

  // Read the file
  let plugin_code = fs::read_to_string(plugin_path).unwrap();

  // Execute
  match win.eval(&plugin_code) {
    Ok(_) => {
      success!("Loaded plugin: {}", name)
    },
    Err(e) => {
      error!("Failed to load plugin: {}", e);
    }
  };
}

/// Load all plugins
#[tauri::command]
pub fn load_all_plugins(win: tauri::WebviewWindow) {
  let mod_config = get_mod_config().unwrap();
  let plugins = mod_config.load_order.iter().filter(|x| x.starts_with("plugins/") && !mod_config.disabled.contains(x)).map(|x| x.replace("plugins/", "")).collect::<Vec<_>>();

  for plugin in plugins {
    load_plugin(&win, plugin);
  }
}

/// Get list of plugins 
#[tauri::command]
pub fn get_plugin_list() -> Vec<String> {
  let plugins_path = get_plugins_path();
  let mut mods = vec![];

  if let Ok(entries) = fs::read_dir(&plugins_path) {
    for entry in entries.flatten() {
      // Check if dir, continue if not
      if !entry.file_type().unwrap().is_file() {
        continue;
      }

      let name = format!("plugins/{}", entry.file_name().to_string_lossy().to_string());

      mods.push(name);
    }
  } else {
    // Create folder
    warn!("Creating plugins folder");
    fs::create_dir(plugins_path).unwrap_or_default();
  }

  mods
}