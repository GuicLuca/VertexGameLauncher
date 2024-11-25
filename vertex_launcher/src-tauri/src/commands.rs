use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest::Client;
use tauri_plugin_http::reqwest::header::ACCEPT;
use futures_util::stream::StreamExt;
use crate::errors::Verror;
use crate::env::LOCAL_GAME_LIST;
use crate::errors;
use crate::errors::Verror::{GameListFetchError, MessageError};
use crate::games::Game;

/// # Commands module
/// This module contains the commands that can be invoked from the frontend.<br>
/// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command<br>
/// **WARNING:** When adding a new command, you must also update the invoke_handler in src-tauri/src/lib.rs


///## Greet command
/// **Description**: Greet the user. This function is not intended to be use but is kept as example.<br>
/// **Frontend usage**:
/// ```typescript
/// invoke('greet', {name: 'John'})
/// .then((msg) => {console.log(msg)})
/// ```
/// **Parameters**:<br>
/// NAME (TYPE)\[SOURCE]: DESCRIPTION
/// - name (String)\[FrontEnd]: The name of the person to greet.
/// 
/// **Returns**:
/// - String : The greeting message. e.g. "Hello, John! You've been greeted from Rust!"
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

///## Get launcher version command
/// **Description**: Get the version of the launcher. Version fetch from the cargo.toml "package" variable".<br>
/// **Frontend usage**:
/// ```typescript
/// invoke('get_launcher_version')
/// .then((version) => {console.log(version)})
/// ```
/// **Returns**:
/// - String : The version of the launcher. e.g. "0.1.0"
#[tauri::command]
pub fn get_launcher_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    version.to_string()
}

///## Get game list command
/// **Description**: Get the list of games.<br>
/// **Frontend usage**:
/// ```typescript
/// invoke('get_game_list')
/// .then((gameList) => {
///    let games = JSON.parse(gameList);
///    // use the games list ...
/// })
/// .catch((error) => {
///   console.error(error);
/// });
/// ```
/// 
/// **Returns**:
/// - String : The list of games in JSON format. e.g. "\[{...}, {...}, ...]"
#[tauri::command]
pub async fn get_game_list() -> Result<String, Verror> {
    let game_list = LOCAL_GAME_LIST.read().await;
    
    // Serialize the games list to an array of games
    let games: Vec<Game> = game_list.values().cloned().collect();
    let games_json = serde_json::to_string(&games)?;
    
    Ok(games_json)
}

/// ## Download command
/// **Description**: Download a file from the internet.<br>
/// **Frontend usage**:
/// ```typescript
/// invoke('download', {game: id})
/// .then(() => {
///   // download completed
/// })
/// .catch((error) => {
///  console.error(error);
/// });
/// ```
///
/// **Parameters**:<br>
/// NAME (TYPE)\[SOURCE]: DESCRIPTION
/// - app_handle (AppHandle)\[tauri-Backend]: The handle to the application used to access the store.<br>
///
/// **Returns**:
/// - String : The list of games in JSON format. e.g. "\[{...}, {...}, ...]"
#[tauri::command]
pub async fn download(app_handle: AppHandle, game: u8) -> errors::Result<()> {
    // 1- Get the game from the local game list
    let local_game = {
        let game_list = LOCAL_GAME_LIST.read().await;
        game_list.get(&game).ok_or(GameListFetchError(format!("Game with id {} not found", game)))?.to_owned()
    };

    // 2- create the folder to store the downloaded file
    let game_data_folder = app_handle.path().app_data_dir()?.join(local_game.get_folder_name());
    let zip_path = game_data_folder.join(&local_game.download_link.name);
    fs::create_dir_all(&game_data_folder)?;
    
    let start_time = Instant::now();
    let client = Client::new();
    let response = client
        .get(&local_game.download_link.url)
        .header(ACCEPT, "application/octet-stream")
        .send()
        .await?;
    
    let total_size = response
        .content_length().ok_or(MessageError("Failed to get the content length".to_string()))?;

    let mut file = File::create(&zip_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    while let Some(stream_item) = stream.next().await {
        let chunk = stream_item.or(Err(MessageError(format!("Failed to get a chunk for game {}", &local_game.title))))?;
        file.write(&chunk)?;
        
        downloaded = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        
        let duration = start_time.elapsed().as_secs_f64();
        let speed = if duration > 0.0 {
            Some(downloaded as f64 / duration / 1024.0 / 1024.0)
        } else {
            None
        };
        
        println!("downloaded => {}", downloaded);
        println!("total_size => {}", total_size);
        println!("speed => {:?}", speed);
    }
    
    // download completed
    let mut game_list = LOCAL_GAME_LIST.write().await;
    let update_local_game = game_list.get_mut(&game).ok_or(GameListFetchError(format!("Game with id {} not found", game)))?;
    update_local_game.download_link.local_path = Some(zip_path);

    Ok(())
}
