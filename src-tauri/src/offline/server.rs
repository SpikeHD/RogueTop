use std::str::FromStr;
use tiny_http::{Header, Response, Server};
use mime_guess::from_path;

use crate::{log, warn};
use super::game::GameFiles;

pub fn start_server() {
  let server = Server::http("127.0.0.1:7653").expect("failed to create local server");

  for request in server.incoming_requests() {
    let path = request.url().strip_prefix('/').unwrap_or(request.url());
    let actual_path = if path == "/" || path == "" {
      "index.html"
    } else {
      path.clone()
    };

    let file = GameFiles::get(actual_path);

    if let Some(file) = file {
      log!("Serving file: {}", actual_path);

      let file = file.data;
      let mime = from_path(actual_path).first_or_text_plain();
      let mut response = Response::from_data(file);
      let content_type = Header::from_str(&format!("Content-Type: {}", mime)).unwrap();
      
      response.add_header(content_type);

      request.respond(response).expect("failed to respond to request");
    } else {
      warn!("File not found: {}", actual_path);

      let response = Response::empty(404);
      request.respond(response).expect("failed to respond to request");
    }
  }
}