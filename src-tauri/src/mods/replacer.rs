use std::{fs, path::PathBuf};

use crate::{util::paths::get_replacers_path, warn};

/// Get a file
pub fn get_alt_file(path: PathBuf) -> Result<Vec<u8>, std::io::Error> {
  // Path passed in should be relative to the mods/textures directory
  let replacers_path = get_replacers_path();

  // For each mod in the load order, check if the file exists
  for mod_name in get_replacer_list() {
    let mod_path = replacers_path.join(&mod_name).join(&path);

    if fs::metadata(&mod_path).is_ok() {
      return fs::read(&mod_path);
    }
  }

  Err(std::io::Error::new(
    std::io::ErrorKind::NotFound,
    "File not found",
  ))
}

/// Get mod list
#[tauri::command]
pub fn get_replacer_list() -> Vec<String> {
  let replacers_path = get_replacers_path();
  let mut mods = vec![];

  if let Ok(entries) = fs::read_dir(&replacers_path) {
    for entry in entries.flatten() {
      // Check if dir, continue if not
      if !entry.file_type().unwrap().is_dir() {
        continue;
      }

      let name = format!("replacers/{}", entry.file_name().to_string_lossy().to_string());

      mods.push(name);
    }
  } else {
    warn!("Creating replacers folder");
    fs::create_dir(replacers_path).unwrap_or_default();
  }

  mods
}
