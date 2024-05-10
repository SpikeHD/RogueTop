use std::{
  fs::{self, File},
  io::Read,
  path::PathBuf,
  sync::{Mutex, OnceLock},
};
use zip::{self, ZipArchive};

use crate::{log, warn};

static FILE_HANDLE: Mutex<Option<ZipArchive<File>>> = Mutex::new(None);

pub fn init(path: &PathBuf) -> bool {
  let file = match fs::File::open(path) {
    Ok(f) => f,
    Err(e) => {
      eprintln!("Failed to open file: {}", e);
      return false;
    }
  };

  let file = match zip::ZipArchive::new(file) {
    Ok(f) => f,
    Err(e) => {
      eprintln!("Failed to open zip archive: {}", e);
      return false;
    }
  };

  FILE_HANDLE.lock().unwrap().replace(file);

  true
}

/// Get a file from the zip archive
pub fn get_file(path: String) -> Result<Vec<u8>, std::io::Error> {
  let mut file = FILE_HANDLE.lock().unwrap();
  let archive = file.as_mut().unwrap();

  log!("Getting file: {}", path);

  let mut file = archive.by_name(&path)?;
  let mut data = Vec::new();
  file.read_to_end(&mut data)?;

  Ok(data)
}
