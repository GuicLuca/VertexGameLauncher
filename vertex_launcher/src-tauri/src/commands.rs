/// # Commands module
/// This module contains the commands that can be invoked from the frontend.<br>
/// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command<br>
/// **WARNING:** When adding a new command, you must also update the invoke_handler in src-tauri/src/lib.rs

use crate::errors::Verror;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use crate::env;

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
/// **Description**: Get the list of games from the local store.<br>
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
/// **Parameters**:<br>
/// NAME (TYPE)\[SOURCE]: DESCRIPTION
/// - app_handle (AppHandle)\[tauri-Backend]: The handle to the application used to access the store.<br>
/// 
/// **Returns**:
/// - String : The list of games in JSON format. e.g. "\[{...}, {...}, ...]"
#[tauri::command]
pub fn get_game_list(app_handle: AppHandle) -> Result<String, Verror> {
    //fetch the game list from the store
    let store = match app_handle.store(env::STORE_FILE_NAME) {
        Ok(store) => store,
        Err(e) => {
            eprintln!("Error fetching store from get_name_list command: {:?}", e);
            return Err(Verror::StoreAccessError(
                "get_game_list command".to_string(),
            ));
        }
    };

    match store.get(env::STORE_GAME_LIST_KEY) {
        None => Ok("[]".to_string()),
        Some(game_list) => Ok(game_list.to_string()),
    }
}
