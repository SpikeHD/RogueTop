#[tauri::command]
pub fn supports_offline() -> bool {
  cfg!(feature = "offline")
}