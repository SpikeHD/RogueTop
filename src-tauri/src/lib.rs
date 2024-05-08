mod config;
mod util;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let _config = config::get_config();

  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
