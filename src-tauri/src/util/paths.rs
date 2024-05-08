use std::{fs, path::PathBuf};

use crate::config::{self};
use crate::log;

pub fn get_config_dir() -> PathBuf {
  // First check for a local config file
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  if fs::metadata(&local_config_dir).is_ok() {
    log!("Using local config file");
    return local_config_dir;
  }

  #[cfg(target_os = "windows")]
  let appdata = dirs::data_dir().unwrap_or_default();

  #[cfg(not(target_os = "windows"))]
  let appdata = dirs::config_dir().unwrap_or_default();

  let config_file = appdata.join("roguetop").join("config.json");

  if fs::metadata(appdata.join("roguetop")).is_err() {
    fs::create_dir_all(appdata.join("roguetop")).expect("Error creating appdata dir");
  }

  // Write default config if it doesn't exist
  if fs::metadata(&config_file).is_err() {
    fs::write(
      &config_file,
      serde_json::to_string(&config::Config::default()).unwrap_or("{}".to_string()),
    )
    .unwrap_or(());
  }

  config_file
}

pub fn config_is_local() -> bool {
  let current_exe = std::env::current_exe().unwrap_or_default();
  let local_config_dir = current_exe.parent().unwrap().join("config.json");

  fs::metadata(local_config_dir).is_ok()
}
