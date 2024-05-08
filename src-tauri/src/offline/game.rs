use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../src-ext/dist"]
pub struct GameFiles;