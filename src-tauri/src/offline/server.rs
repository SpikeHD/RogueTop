use mime_guess::from_path;
use std::io::Read;
use std::str::FromStr;
use tiny_http::{Header, Response, Server};
use url_escape::decode;

use tauri::{path::BaseDirectory, Manager};

use crate::{log, warn};

pub fn start_server(app: tauri::AppHandle) {
  let server = Server::http("127.0.0.1:7653").expect("failed to create local server");

  for request in server.incoming_requests() {
    let path = request.url().strip_prefix('/').unwrap_or(request.url());
    let actual_path = if path == "/" || path == "" {
      "index.html"
    } else {
      path
    };

    let file = app.path().resolve(
      format!("../src-ext/dist/{}", decode(actual_path)),
      BaseDirectory::Resource,
    );

    if let Ok(file) = file {
      let file = match std::fs::File::open(&file) {
        Ok(f) => f,
        Err(e) => {
          warn!("Failed to open file: {}", e);
          let response = Response::empty(500);
          request
            .respond(response)
            .expect("failed to respond to request");
          continue;
        }
      };
      let data = std::io::BufReader::new(file);
      let data = data
        .bytes()
        .collect::<Result<Vec<u8>, _>>()
        .expect("failed to read file");
      let mime = from_path(actual_path).first_or_text_plain();
      let mut response = Response::from_data(data);
      let content_type = Header::from_str(&format!("Content-Type: {}", mime)).unwrap();

      response.add_header(content_type);

      request
        .respond(response)
        .expect("failed to respond to request");
    } else {
      warn!("File not found: {}", actual_path);

      let response = Response::empty(404);
      request
        .respond(response)
        .expect("failed to respond to request");
    }
  }
}
