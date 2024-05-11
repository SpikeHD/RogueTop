pub mod log;
pub mod paths;
pub mod support;

#[tauri::command]
pub fn is_dev() -> bool {
  cfg!(debug_assertions)
}