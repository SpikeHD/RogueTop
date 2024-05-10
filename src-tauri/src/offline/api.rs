use tiny_http::{Header, Request, Response, Server};

use crate::log;

pub fn handle_request(request: Request) {
  let body = request.as_reader().collect::<Result<Vec<u8>, _>>().unwrap();

  log!("Handling request: {:?}", request.url());
  log!("Body: {:?}", body);
}