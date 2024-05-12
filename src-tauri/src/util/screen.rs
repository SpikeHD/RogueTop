#[tauri::command]
pub fn toggle_fullscreen(win: tauri::Window) {
  win.set_fullscreen(
    !win.is_fullscreen().unwrap_or_default()
  ).unwrap_or_default();
}