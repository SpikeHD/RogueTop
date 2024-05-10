use tauri::{
  plugin::{Builder, TauriPlugin},
  Runtime,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("init_script")
    .js_init_script(include_str!("../../ext/index_min.js").to_string())
    .build()
}
