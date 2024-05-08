use std::str::FromStr;

use tauri::{Manager, Url};

mod config;
mod game;
mod offline;
mod util;

static REMOTE_URL: &str = "http://pokerogue.net";
static LOCAL_URL: &str = "http://localhost:7653";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let config = config::get_config();
  let mut maybe_navigate: Option<Url> = None;

  if config.skip_splash.unwrap_or(false) {
    // If online, point context to the site, otherwise point to the soon-to-be-running local version
    if config.offline.unwrap_or(false) {
      if let Ok(url) = Url::from_str(LOCAL_URL) {
        maybe_navigate = Some(url);
      }
    } else {
      if let Ok(url) = Url::from_str(REMOTE_URL) {
        maybe_navigate = Some(url);
      }
    }
  }

  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
      config::read_config_file,
      config::write_config_file,
      config::default_config,
      config::get_config,
      game::launch
    ])
    .setup(move |app| {
      if let Some(navigate) = maybe_navigate {
        if let Some(mut win) = app.get_webview_window("main") {
          win.navigate(navigate);

          // If we are offline, start the offline server in a new thread
          #[cfg(feature = "offline")]
          if config.offline.unwrap_or(false) {
            std::thread::spawn(|| {
              offline::server::start_server();
            });
          }
        }
      }

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running application");
}
