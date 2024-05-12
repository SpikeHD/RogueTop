#[tauri::command]
pub fn supports_offline() -> bool {
  cfg!(feature = "offline")
}

#[tauri::command]
pub fn is_mobile() -> bool {
  // Check if we are an iOS or android target
  cfg!(target_os = "ios") || cfg!(target_os = "android")
}

#[tauri::command]
pub fn is_ios() -> bool {
  cfg!(target_os = "ios")
}

#[tauri::command]
pub fn is_android() -> bool {
  cfg!(target_os = "android")
}