use std::str::FromStr;
use tauri::Manager;

use crate::config;
#[cfg(feature = "offline")]
use crate::offline;

#[tauri::command]
pub fn launch(app: tauri::AppHandle) -> bool {
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

  if let Some(mut win) = app.get_webview_window("main") {
    win.navigate(url);
  }

  // If we are offline, start the offline server in a new thread
  #[cfg(feature = "offline")]
  if config.offline.unwrap_or(false) {
    std::thread::spawn(|| {
      offline::server::start_server();
    });
  }

  true
}
