use serde::{Deserialize, Serialize};
use tiny_http::Request;

use crate::{log, warn, LOCAL_URL};

#[derive(Serialize, Deserialize)]
struct Options {
  body: Option<String>,
  method: Option<String>,
}

#[tauri::command]
pub fn api_request(url: String, options: String) -> String {
  let client = reqwest::blocking::Client::new();
  let options: Options = serde_json::from_str(&options).unwrap();
  let body = options.body.clone().unwrap_or("".to_string());
  let method = options.method.clone().unwrap_or("GET".to_string());

  // Split the path from the full URL
  let path = url
    .split_once("http://")
    .unwrap_or(("", ""))
    .1
    .split_once("/")
    .unwrap_or(("", ""))
    .1;
  let url = format!("{}/api/{}", LOCAL_URL, path);

  let response = match method.as_str() {
    "GET" => client.get(&url).send(),
    "POST" => client.post(&url).body(body).send(),
    "PUT" => client.put(&url).body(body).send(),
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

pub fn handle_request(request: &mut Request) -> String {
  let mut body = String::new();

  request.as_reader().read_to_string(&mut body).unwrap();

  let path = request
    .url()
    .split_once("api/")
    .unwrap_or(("", ""))
    .1
    .split_once("/")
    .unwrap_or(("", ""))
    .0;

  match path {
    _ => {
      warn!("Unimplemented API path: {}", path);
      "".to_string()
    }
  }
}
