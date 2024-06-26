pub mod discord_rpc;
pub mod log;
pub mod paths;
pub mod screen;
pub mod support;

#[tauri::command]
pub fn is_dev() -> bool {
  cfg!(debug_assertions)
}
