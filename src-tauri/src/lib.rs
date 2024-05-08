mod config;
mod util;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let config = config::get_config();

  if config.offline.unwrap_or(false) {
    warn!("Offline mode is unimplemented!");
  }

  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
