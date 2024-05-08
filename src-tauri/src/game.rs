use std::str::FromStr;
use tauri::Manager;

use crate::config;

#[tauri::command]
pub fn launch(win: tauri::Window) -> bool {
  let config = config::get_config();
  let url;

  if config.offline.unwrap_or(false) {
    match tauri::Url::from_str(super::LOCAL_URL) {
      Ok(u) => {
        url = u;
      }
      Err(e) => {
        eprintln!("Error parsing URL: {}", e);
        return false;
      }
    }
  } else {
    match tauri::Url::from_str(super::REMOTE_URL) {
      Ok(u) => {
        url = u;
      }
      Err(e) => {
        eprintln!("Error parsing URL: {}", e);
        return false;
      }
    }
  }

  if let Some(mut win) = win.get_webview_window("main") {
    win.navigate(url);
  }

  true
}