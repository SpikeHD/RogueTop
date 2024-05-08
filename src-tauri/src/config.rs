use serde::{Deserialize, Serialize};
use std::fs;

use crate::log;
use crate::util::paths::get_config_dir;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
  skip_splash: Option<bool>,
  offline: Option<bool>,
}

pub fn init() {
  get_config_dir();
}

#[tauri::command]
pub fn read_config_file() -> String {
  init();

  let config_file = get_config_dir();

  fs::read_to_string(config_file).expect("Config does not exist!")
}

#[tauri::command]
pub fn write_config_file(contents: String) {
  init();

  let config_file = get_config_dir();

  fs::write(config_file, contents).expect("Error writing config!")
}

#[tauri::command]
pub fn default_config() -> Config {
  Config {
    skip_splash: Some(false),
    offline: Some(false),
  }
}

pub fn get_config() -> Config {
  let config_str = read_config_file();
  let config_str = config_str.as_str();

  match serde_json::from_str(config_str) {
    Ok(config) => config,
    Err(e) => {
      log!("Failed to parse config, using default config!");
      log!("Error: {}", e);

      default_config()
    }
  }
}
