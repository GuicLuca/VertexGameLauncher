use crate::download::DownloadSteps::Downloading;
use crate::download::GameDownload;
use crate::env::{LOCAL_GAME_LIST, UPDATE_RATE};
use crate::errors::Verror;
use crate::errors::Verror::{GameLaunchError, GameListFetchError, MessageError};
use crate::games::Game;
use crate::{env, errors};
use futures_util::stream::StreamExt;
use log::info;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::time::Instant;
use tauri::{Emitter, Manager};
use tauri_plugin_http::reqwest::header::ACCEPT;
use tauri_plugin_http::reqwest::Client;
use tauri_plugin_store::StoreExt;

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
    let mut games: Vec<Game> = game_list.values().cloned().collect();
    // Order games by weight (Higher first)
    games.sort_by(|a, b| a.weight.cmp(&b.weight).reverse());
    let games_json = serde_json::to_string(&games)?;

    Ok(games_json)
}

///## Get game command
/// **Description**: Get a specific game by its ID from the local game list.<br>
/// **Frontend usage**:
/// ```typescript
/// invoke('get_game', {game: id})
/// .then((gameData) => {
///    let game = JSON.parse(gameData);
///    // use the game data ...
/// })
/// .catch((error) => {
///   console.error(error);
/// });
/// ```
///
/// **Parameters**:<br>
/// NAME (TYPE)\[SOURCE]: DESCRIPTION
/// - game (u8)\[FrontEnd]: The ID of the game to retrieve.
///
/// **Returns**:
/// - Result<String, Verror>: JSON string representing the game data on success, or an error if game is not found
#[tauri::command]
pub async fn get_game(game: u8) -> Result<String, Verror> {
    let game_list = LOCAL_GAME_LIST.read().await;

    match game_list.get(&game) {
        Some(game_data) => {
            let game_json = serde_json::to_string(&game_data)?;
            Ok(game_json)
        }
        None => Err(GameListFetchError(format!(
            "Game with id {} not found",
            game
        ))),
    }
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
/// - game (u8)\[FrontEnd]: The id of the game to download.
#[tauri::command]
pub async fn download(app_handle: tauri::AppHandle, game: u8) -> errors::Result<()> {
    info!("Downloading game {}", game);
    let mut download = GameDownload::new(game, app_handle.clone());

    // 1- Get the game from the local game list
    let local_game = {
        let game_list = LOCAL_GAME_LIST.read().await;
        game_list
            .get(&game)
            .ok_or(GameListFetchError(format!(
                "Game with id {} not found",
                game
            )))?
            .to_owned()
    };

    // 2- create the folder to store the downloaded file
    let game_data_folder = app_handle
        .path()
        .app_data_dir()?
        .join(local_game.get_folder_name());
    let archive_path = game_data_folder.join(&local_game.game_archive.link.name);
    fs::create_dir_all(&game_data_folder)?;

    // 3- Download the zip file
    let start_time = Instant::now();
    let client = Client::new();
    let response = client
        .get(&local_game.game_archive.link.url)
        .header(ACCEPT, "application/octet-stream")
        .send()
        .await?;

    let total_size = response
        .content_length()
        .ok_or(MessageError("Failed to get the content length".to_string()))?;

    download.set_file_size(total_size);
    download.set_start_time(start_time);
    download.set_steps(Downloading);

    let mut last_update = Instant::now() - std::time::Duration::from_millis(UPDATE_RATE as u64);

    let mut bytes: Vec<u8> = Vec::with_capacity(total_size as usize);
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(stream_item) = stream.next().await {
        let chunk = stream_item.or(Err(MessageError(format!(
            "Failed to get a chunk for game {}",
            &local_game.title
        ))))?;

        downloaded = std::cmp::min(downloaded + (chunk.len() as u64), total_size);

        bytes.append(&mut chunk.to_vec());

        download.update(downloaded, None);

        if (last_update.elapsed().as_millis() as u16) < UPDATE_RATE {
            // don't advertise the download progress too often
            continue;
        }
        // Advertise the download progress
        download.advertise();
        info!("Download progress: {}", download.get_state());

        last_update = Instant::now();
    }
    // advertise a last time to get the 100% of progress
    download.advertise();

    // download completed
    info!(
        "Download completed in {:.2} seconds",
        start_time.elapsed().as_secs_f64()
    );

    // 4 - Extract the zip file to the game folder
    if local_game.game_archive.need_extract {
        download.set_steps(crate::download::DownloadSteps::Extracting);

        zip_extract::extract(
            Cursor::new(bytes),
            &game_data_folder,
            local_game.game_archive.strip_top_level_folder,
        )?;

        // 5 - Update the local game list with the downloaded file path
        let mut game_list = LOCAL_GAME_LIST.write().await;
        let update_local_game = game_list.get_mut(&game).ok_or(GameListFetchError(format!(
            "Game with id {} not found",
            game
        )))?;
        update_local_game.game_archive.link.local_path = Some(
            game_data_folder
                .join(PathBuf::from(&local_game.game_archive.path_to_executable).as_os_str()),
        );

        // 6 - Delete the zip file
        info!("Cleaning downloaded files");
        download.set_steps(crate::download::DownloadSteps::Cleaning);
    } else {
        let mut game_list = LOCAL_GAME_LIST.write().await;
        let update_local_game = game_list.get_mut(&game).ok_or(GameListFetchError(format!(
            "Game with id {} not found",
            game
        )))?;
        update_local_game.game_archive.link.local_path = Some(
            archive_path
                .join(PathBuf::from(&local_game.game_archive.path_to_executable).as_os_str()),
        );
    }

    // 7 - Update the local game list and give it to the frontend
    {
        let store = match app_handle.store(env::STORE_FILE_NAME) {
            Ok(store) => store,
            Err(e) => return Err(errors::Verror::StoreAccessError(e.to_string())),
        };
        let game_list = LOCAL_GAME_LIST.read().await;
        store.set(
            env::STORE_LOCAL_GAME_LIST_KEY,
            serde_json::to_value(&*game_list).unwrap(),
        );
    }
    // Use the get game list command to update the frontend ensuring the format is always the same for the frontend
    app_handle.emit(env::EVENT_GAME_LIST_UPDATED, get_game_list().await?)?;

    download.complete().await;
    Ok(())
}


/// ## Launch a game
/// **Description**: Launch a game using its executable file.<br>
/// **Frontend usage**:
/// ```typescript
/// invoke('launch', {game: id})
/// .then(() => {
///   // game process is terminated
/// })
/// .catch((error) => {
///  console.error(error);
/// });
/// ```
///
/// **Parameters**:<br>
/// NAME (TYPE)\[SOURCE]: DESCRIPTION
/// - app_handle (AppHandle)\[tauri-Backend]: The handle to the application used to access the store.<br>
/// - game (u8)\[FrontEnd]: The id of the game to launch.
#[tauri::command]
pub async fn launch(app_handle: tauri::AppHandle, game: u8) -> errors::Result<()> {
    // 1 - ensure the game is downloaded and the executable file exists
    let executable_path = {
        let game_list = LOCAL_GAME_LIST.read().await;
        let local_game = game_list.get(&game).ok_or(GameListFetchError(format!(
            "Game with id {} not found",
            game
        )))?;
        let game_path =
            local_game
                .game_archive
                .link
                .local_path
                .as_ref()
                .ok_or(GameListFetchError(format!(
                    "Game with id {} not found",
                    game
                )))?;
        if !game_path.exists() {
            return Err(GameLaunchError(format!(
                "Executable file for game {} not found",
                game
            )));
        }
        game_path.to_owned()
    };

    // 2 - Launch the game
    match std::process::Command::new(executable_path).spawn() {
        Err(e) => {
            return Err(GameLaunchError(format!(
                "Failed to launch game {}: {}",
                game, e
            )))
        }
        Ok(mut child) => {
            info!("Game {} launched", game);
            // wait for the game to finish
            match child.wait() {
                Ok(status) => {
                    info!("Game {} terminated with status: {}", game, status);

                    // Broadcast the game termination
                    app_handle.emit(
                        &format!("{}_{}", env::EVENT_GAME_PROCESS_TERMINATED, game),
                        status.code(),
                    )?;
                }
                Err(e) => {
                    return Err(GameLaunchError(format!(
                        "Failed to wait for game {} to finish: {}",
                        game, e
                    )));
                }
            }
        }
    }

    Ok(())
}
