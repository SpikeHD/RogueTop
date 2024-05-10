mod config;
mod game;
mod init_plugin;
#[cfg(feature = "offline")]
mod offline;
mod util;

static REMOTE_URL: &str = "https://pokerogue.net";
static LOCAL_URL: &str = "http://localhost:7653";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let config = config::get_config();

  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(init_plugin::init())
    .invoke_handler(tauri::generate_handler![
      #[cfg(feature = "offline")]
      offline::api::api_request,
      util::support::supports_offline,
      config::read_config_file,
      config::write_config_file,
      config::default_config,
      config::get_config,
      game::launch
    ])
    .setup(move |app| {
      if config.skip_splash.unwrap_or(false) {
        if config.offline.unwrap_or(false) {
          #[cfg(feature = "offline")]
          game::launch(app.handle().clone());

          #[cfg(not(feature = "offline"))]
          {
            warn!(
              "Offline mode requested, but feature is not enabled. Opening options panel instead."
            );

            // Write to the config that we should run in online mode
            let mut config = config::get_config();
            config.offline = Some(false);
            let config_str = serde_json::to_string(&config).expect("Failed to serialize config!");

            config::write_config_file(config_str);
          }
        } else {
          game::launch(app.handle().clone());
        }
      }

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running application");
}
