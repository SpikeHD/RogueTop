use std::sync::Mutex;

use discord_rich_presence::{
  activity,
  DiscordIpc,
  DiscordIpcClient
};

static CLIENT_ID: &str = "1238914884586962974";
static CLIENT: Mutex<Option<DiscordIpcClient>> = Mutex::new(None);

pub fn connect_discord_rpc() -> Result<(), Box<dyn std::error::Error>> {
  let mut client = DiscordIpcClient::new(CLIENT_ID)?;
  client.connect()?;
  
  let mut global_client = CLIENT.lock().unwrap();
  *global_client = Some(client);

  Ok(())
}

#[tauri::command]
pub fn rpc_enabled() -> bool {
  CLIENT.lock().unwrap().is_some()
}

// TODO - maybe add images?
#[tauri::command]
pub fn set_activity(
  state: &str,
  details: &str,
  large_text: Option<&str>,
  large_image: Option<&str>,
  small_text: Option<&str>,
  small_image: Option<&str>
) {
  let mut client = CLIENT.lock().unwrap();
  let client = match client.as_mut() {
    Some(client) => client,
    // If none, client failed to connect
    None => return
  };

  let activity = activity::Activity::new()
    .state(state)
    .details(details)
    .assets(
      activity::Assets::new()
        .large_text(large_text.unwrap_or_default())
        .large_image(large_image.unwrap_or_default())
        .small_text(small_text.unwrap_or_default())
        .small_image(small_image.unwrap_or_default())
    );

  (*client).set_activity(activity).unwrap_or_default();
}

pub fn remove_activity() {
  let mut client = CLIENT.lock().unwrap();
  let client = match client.as_mut() {
    Some(client) => client,
    // If none, client failed to connect
    None => return
  };

  (*client).clear_activity().unwrap_or_default();
}