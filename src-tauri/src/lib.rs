use tauri::menu::{AboutMetadata, Menu, PredefinedMenuItem, Submenu};
use util::discord_rpc::connect_discord_rpc;

mod config;
mod game;
mod init_plugin;
mod mods;
#[cfg(feature = "offline")]
mod offline;
mod util;

static REMOTE_URL: &str = "https://pokerogue.net";
static LOCAL_URL: &str = "http://localhost:7653";

fn build_menu<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<Menu<R>> {
  let pkg_info = app.package_info();
  let config = app.config();

  let credits = format!(
    "PokeRogue: {} ({})\nRogueTop: {}",
    env!("POKEROGUE_VERSION"),
    env!("POKEROGUE_COMMIT"),
    env!("ROGUETOP_COMMIT"),
  );

  let about_metadata = AboutMetadata {
    name: Some(pkg_info.name.clone()),
    version: Some(pkg_info.version.to_string()),
    copyright: config.bundle.copyright.clone(),
    authors: config.bundle.publisher.clone().map(|p| vec![p]),
    credits: Some(credits.clone()),
    comments: Some(credits),
    ..Default::default()
  };

  let window_menu = Submenu::with_items(
    app,
    "Window",
    true,
    &[
      &PredefinedMenuItem::minimize(app, None)?,
      &PredefinedMenuItem::maximize(app, None)?,
      #[cfg(target_os = "macos")]
      &PredefinedMenuItem::separator(app)?,
      &PredefinedMenuItem::close_window(app, None)?,
    ],
  )?;

  let help_menu = Submenu::with_items(
    app,
    "Help",
    true,
    &[
      #[cfg(not(target_os = "macos"))]
      &PredefinedMenuItem::about(app, None, Some(about_metadata.clone()))?,
    ],
  )?;

  Menu::with_items(
    app,
    &[
      #[cfg(target_os = "macos")]
      &Submenu::with_items(
        app,
        pkg_info.name.clone(),
        true,
        &[
          &PredefinedMenuItem::about(app, None, Some(about_metadata))?,
          &PredefinedMenuItem::separator(app)?,
          &PredefinedMenuItem::services(app, None)?,
          &PredefinedMenuItem::separator(app)?,
          &PredefinedMenuItem::hide(app, None)?,
          &PredefinedMenuItem::hide_others(app, None)?,
          &PredefinedMenuItem::separator(app)?,
          &PredefinedMenuItem::quit(app, None)?,
        ],
      )?,
      #[cfg(not(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
      )))]
      &Submenu::with_items(
        app,
        "File",
        true,
        &[
          &PredefinedMenuItem::close_window(app, None)?,
          #[cfg(not(target_os = "macos"))]
          &PredefinedMenuItem::quit(app, None)?,
        ],
      )?,
      &Submenu::with_items(
        app,
        "Edit",
        true,
        &[
          &PredefinedMenuItem::undo(app, None)?,
          &PredefinedMenuItem::redo(app, None)?,
          &PredefinedMenuItem::separator(app)?,
          &PredefinedMenuItem::cut(app, None)?,
          &PredefinedMenuItem::copy(app, None)?,
          &PredefinedMenuItem::paste(app, None)?,
          &PredefinedMenuItem::select_all(app, None)?,
        ],
      )?,
      #[cfg(target_os = "macos")]
      &Submenu::with_items(
        app,
        "View",
        true,
        &[&PredefinedMenuItem::fullscreen(app, None)?],
      )?,
      &window_menu,
      &help_menu,
    ],
  )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let config = config::get_config();

  tauri::Builder::default()
    .menu(build_menu)
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_gamepad::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(init_plugin::init())
    .invoke_handler(tauri::generate_handler![
      #[cfg(feature = "offline")]
      offline::api::api_request,
      util::is_dev,
      util::screen::toggle_fullscreen,
      util::support::supports_offline,
      util::discord_rpc::set_activity,
      util::discord_rpc::rpc_enabled,
      config::read_config_file,
      config::write_config_file,
      config::default_config,
      config::get_config,
      game::launch,
      mods::open_mods_folder,
      mods::get_mods_list,
      mods::plugin::load_all_plugins
    ])
    .on_window_event(|_window, event| match event {
      tauri::WindowEvent::Destroyed => {
        util::discord_rpc::remove_activity();
      }
      tauri::WindowEvent::CloseRequested { .. } => {
        util::discord_rpc::remove_activity();
      }
      _ => {}
    })
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

      if config.rpc.unwrap_or(false) {
        connect_discord_rpc().unwrap_or_else(|e| {
          error!("Failed to connect to Discord RPC: {}", e);
        });
      }

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running application");
}
