use serde::{Deserialize, Serialize};
use tiny_http::{Header, Request, Response, Server};

use crate::{log, warn, LOCAL_URL};

#[derive(Serialize, Deserialize)]
struct Options {
  body: String,
  method: String,
}

#[tauri::command]
pub fn api_request(url: String, options: String) -> String {
  let client = reqwest::blocking::Client::new();
  let options: Options = serde_json::from_str(&options).unwrap();

  // Split the path from the full URL
  let path = url.split_once("/").unwrap_or(("", "")).1;
  let url = format!("{}/api{}", LOCAL_URL, path);

  log!("{}", url);

  let response = match options.method.as_str() {
    "GET" => client.get(&url).send(),
    "POST" => client.post(&url).body(options.body).send(),
    "PUT" => client.put(&url).body(options.body).send(),
    "DELETE" => client.delete(&url).send(),
    _ => client.get(&url).send(),
  };

  match response {
    Ok(response) => {
      let text = response.text().unwrap();
      log!("Response: {:?}", text);

      text
    }
    Err(e) => {
      warn!("Error: {:?}", e);

      "".to_string()
    }
  }
}

pub fn handle_request(request: &mut Request) {
  let mut body = String::new();

  request.as_reader().read_to_string(&mut body).unwrap();

  log!("Handling request: {:?}", request.url());
  log!("Body: {:?}", body);
}
