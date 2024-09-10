#[tauri::command]
pub fn toggle_fullscreen(win: tauri::Window) {
  #[cfg(not(any(target_os = "android", target_os = "ios")))]
  win
    .set_fullscreen(!win.is_fullscreen().unwrap_or_default())
    .unwrap_or_default();

  #[cfg(any(target_os = "android", target_os = "ios"))]
  return;
}
