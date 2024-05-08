use rust_embed::RustEmbed;

#[cfg(target_os = "windows")]
#[derive(RustEmbed)]
#[folder = r"..\src-ext\dist"]
pub struct GameFiles;

#[cfg(not(target_os = "windows"))]
#[derive(RustEmbed)]
#[folder = "../src-ext/dist"]
pub struct GameFiles;