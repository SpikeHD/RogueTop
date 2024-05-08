mod config;
mod util;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let config = config::get_config();

  if config.offline.unwrap_or(false) {
    #[cfg(not(feature = "offline"))]
    {
      error!("You are running RogueTop Lite, which means you are online-only.")
    }
  }

  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
      config::read_config_file,
      config::write_config_file,
      config::default_config,
    ])
    .run(tauri::generate_context!())
    .expect("error while running application");
}
