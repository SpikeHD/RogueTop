use mime_guess::from_path;
use std::str::FromStr;
use tiny_http::{Header, Response, Server};
use url_escape::decode;

use tauri::{path::BaseDirectory, Manager};

use crate::offline::zip;
use crate::warn;

pub fn start_server(app: tauri::AppHandle) {
  let server = Server::http("127.0.0.1:7653").expect("failed to create local server");

  // Init the file handle for the game files
  let file = app
    .path()
    .resolve("../game.dat", BaseDirectory::Resource)
    .expect("failed to resolve game.dat");

  zip::init(&file);

  for mut request in server.incoming_requests() {
    let path = request.url().strip_prefix('/').unwrap_or(request.url());

    // If this is a request to the API, hand it off
    if path.starts_with("api/") {
      crate::offline::api::handle_request(&mut request);
      continue;
    }

    let actual_path = if path == "/" || path == "" {
      "index.html"
    } else {
      path
    };

    // Strip duplicate slashes
    let actual_path = actual_path.replace("//", "/");

    let file = zip::get_file(decode(&actual_path.to_string()).to_string());

    match file {
      Ok(data) => {
        let mime = from_path(actual_path).first_or_text_plain();
        let mut response = Response::from_data(data);
        let content_type = Header::from_str(&format!("Content-Type: {}", mime)).unwrap();

        response.add_header(content_type);

        request
          .respond(response)
          .expect("failed to respond to request");
      }
      Err(e) => {
        warn!("File not found: {}", path);
        warn!("Error: {:?}", e);

        let response = Response::empty(404);
        request
          .respond(response)
          .expect("failed to respond to request");
      }
    }
  }
}
