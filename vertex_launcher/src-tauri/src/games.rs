use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf};
use log::{error};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use crate::errors;
use crate::errors::Verror::{GameResourceDownloadError, Io, MessageError};
use crate::games::LinkType::{BackgroundImage, GameArchiveLink, NavigationIcon};

/// A struct that represents a link to a resource. It will contain all the information needed to 
/// download the resource and save it to the app's data directory.<br>
/// This struct is also used to check if the resource is already downloaded at the latest revision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub url: String,
    pub name: String,
    pub revision: u64,
    pub local_path: Option<PathBuf>,
}

impl Link {
    pub fn new(url: String, name: String, revision: u64, local_path: Option<PathBuf>) -> Link {
        Link {
            url,
            name,
            revision,
            local_path,
        }
    }
    
    pub fn from_json_object(json_map: &Map<String, Value>) -> errors::Result<Link> {
        let url = json_map["url"].as_str().unwrap().to_string();
        let name = json_map["name"].as_str().unwrap().to_string();
        let revision = json_map["revision"].as_u64().unwrap();
        let local_path: Option<PathBuf> = match json_map.get("local_path") {
            Some(value) => {
                Some(PathBuf::deserialize(value).unwrap())
            },
            None => None
        };
        
        Ok(Link::new(url, name, revision, local_path))
    }

    fn is_json_valid(json: &Value) -> bool {
       let base_validity = json.get("url").is_some() && json["url"].is_string() &&
            json.get("name").is_some() && json["name"].is_string() &&
            json.get("revision").is_some() && json["revision"].is_u64();
        // check if the json contains a local path and if it is a string
        let local_path_validity = match json.get("local_path") {
            Some(value) => value.is_string(),
            None => true
        };
        base_validity && local_path_validity
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameArchive {
    pub link: Link,
    pub need_extract: bool,
    pub strip_top_level_folder: bool,
    pub path_to_executable: String,
    pub need_update: bool,
}

impl GameArchive {
    pub fn new(link: Link, need_extract: bool, strip_top_level_folder: bool, path_to_executable: String) -> GameArchive {
        GameArchive {
            link,
            need_extract,
            strip_top_level_folder,
            path_to_executable,
            need_update: false,
        }
    }

    pub fn from_json_object(json_map: &Map<String, Value>) -> errors::Result<GameArchive> {
        let url = Link::from_json_object(json_map["link"].as_object().unwrap())?;
        let need_extract = json_map["need_extract"].as_bool().unwrap();
        let strip_top_level_folder = json_map["strip_top_level_folder"].as_bool().unwrap();
        let path_to_executable = json_map["path_to_executable"].as_str().unwrap().to_string();

        Ok(GameArchive::new(url, need_extract, strip_top_level_folder, path_to_executable))
    }

    fn is_json_valid(json: &Value) -> bool {
            json.get("link").is_some() && json["link"].is_object() && Link::is_json_valid(&json["link"]) &&
            json.get("need_extract").is_some() && json["need_extract"].is_boolean() &&
            json.get("strip_top_level_folder").is_some() && json["strip_top_level_folder"].is_boolean() &&
            json.get("path_to_executable").is_some() && json["path_to_executable"].is_string()
    }
}

#[allow(dead_code)]
pub enum LinkType {
    BackgroundImage,
    NavigationIcon,
    GameArchiveLink,
}

/// A struct that represents a game. It contains all the information needed to display the game in the launcher.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: u8,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub background_image: Link,
    pub navigation_icon: Link,
    pub game_archive: GameArchive,
    pub version: String,
    pub platform: Vec<String>,
    pub tags: Vec<String>,
}

impl Game {
    fn new(id: u8, title: String, subtitle: String, description: String, background_image: Link,
        navigation_icon: Link, game_archive: GameArchive, version: String, platform: Vec<String>, tags: Vec<String>,
    ) -> Game {
        Game { id, title, subtitle, description, background_image, navigation_icon,
            game_archive,
            version, platform, tags,
        }
    }
    
    /// ##### Initialize a game struct from a json object. 
    /// The json object will be validated before creating the struct.<br>
    /// @param json : The json object that contains the game's information.<br>
    /// @param is_remote_json : A boolean that indicates if the json object is fetched from the internet (true) or from the local store (false).
    pub fn initialize_game_from_json(json: &Value) -> errors::Result<Game> {
        // Create a new game struct from the json data
        if !Self::is_json_valid(json) { 
            return Err(errors::Verror::GameConstructionError(format!("{:?}", json)));
        }
        
        let id = json["id"].as_u64().unwrap() as u8;
        let title = json["title"].as_str().unwrap().to_string();
        let subtitle = json["subtitle"].as_str().unwrap().to_string();
        let description = json["description"].as_str().unwrap().to_string();
        let version = json["version"].as_str().unwrap().to_string();
        let platform = json["platform"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect();
        let tags = json["tags"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect();
        let background_image_link = Link::from_json_object(json["background_image"].as_object().unwrap())?;
        let navigation_icon_link = Link::from_json_object(json["navigation_icon"].as_object().unwrap())?;
        let game_archive = GameArchive::from_json_object(json["download_link"].as_object().unwrap())?;
        
        Ok(Game::new(id, title, subtitle, description, background_image_link, navigation_icon_link, game_archive, version, platform, tags))
    }

    /// Check if the json object is valid for a game struct.
    /// It will call the is_json_valid method of the Link struct to check if the links are also valid.
    fn is_json_valid(json: &Value) -> bool {
        // Check if the json object has all the required fields.
        // it also checks if the fields are of the correct type and with a value for the required fields.
        json.is_object() &&
            json.get("id").is_some() && json["id"].is_u64() &&
            json.get("title").is_some() && json["title"].is_string() &&
            (json.get("subtitle").is_some() || json["subtitle"] == Value::Null) && json["subtitle"].is_string() &&
            (json.get("description").is_some() || json["description"] == Value::Null) && json["description"].is_string() &&
            (json.get("version").is_some() || json["version"] == Value::Null) && json["version"].is_string() &&
            json.get("platform").is_some() && json["platform"].is_array() &&
            (json.get("tags").is_some() || json["tags"] == Value::Null) && json["tags"].is_array() &&
            json.get("background_image").is_some() && json["background_image"].is_object() && Link::is_json_valid(&json["background_image"]) &&
            json.get("navigation_icon").is_some() && json["navigation_icon"].is_object() && Link::is_json_valid(&json["navigation_icon"]) &&
            json.get("download_link").is_some() && json["download_link"].is_object() && GameArchive::is_json_valid(&json["download_link"])
    }
    
    /// Compare the local game with the remote game and perform the necessary actions to update the local game.
    pub async fn update_game(app: &AppHandle, local_game: &mut Game, remote_game: &Game) -> errors::Result<()> {
        local_game.version = remote_game.version.to_owned();
        local_game.title = remote_game.title.to_owned();
        local_game.subtitle = remote_game.subtitle.to_owned();
        local_game.description = remote_game.description.to_owned();
        local_game.platform = remote_game.platform.to_owned();
        local_game.tags = remote_game.tags.to_owned();
        
        // lambda function to update a link
        // return true if the local link need to be downloaded
        let update_link = move |local_link: &mut Link, remote_link: &Link| {
            if local_link.revision < remote_link.revision {
                // update the local link with the remote link
                local_link.url = remote_link.url.to_owned();
                local_link.name = remote_link.name.to_owned();
                local_link.revision = remote_link.revision;
                // if there is a local path, delete the file
                if let Some(local_path) = &local_link.local_path {
                    if let Err(e) = std::fs::remove_file(local_path) {
                        error!("Error deleting file \"{}\" : {:?}", local_path.display(), e);
                    }
                    
                    local_link.local_path = None;
                }
            }
            
            // return true if the local link need to be downloaded
            local_link.local_path.is_none()
        };
        
        
        if update_link(&mut local_game.background_image, &remote_game.background_image) == true {
            local_game.download_link(app, BackgroundImage).await?;
        }
        if update_link(&mut local_game.navigation_icon, &remote_game.navigation_icon) == true {
            local_game.download_link(app, NavigationIcon).await?;
        }
        // Don't download the download link because we want to let the user choose whether to download the game or not.
        // Only say that the download link needs to be updated if the revision is different.
        local_game.game_archive.need_update = update_link(&mut local_game.game_archive.link, &remote_game.game_archive.link);
        
        
        Ok(())
    }
    
    /// Download the link and save it to the app's data directory. <br>
    /// This function requires a mutable reference to the game struct to update the local path of the link.
    async fn download_link(&mut self, app: &AppHandle, link_type: LinkType) -> errors::Result<()> {
        // Get the folder name of the game using self as immutable reference before it gets borrowed as mutable.
        let game_folder_name = self.get_folder_name();
        
        let link = match link_type {
            BackgroundImage => &mut self.background_image,
            NavigationIcon => &mut self.navigation_icon,
            GameArchiveLink => &mut self.game_archive.link,
        };
        
        match reqwest::get(&link.url).await {
            Ok(response) => {
                if response.status().is_success() == false {
                    return Err(GameResourceDownloadError(format!("{:?}", response)));
                }

                let game_data_folder = app.path().app_data_dir()?.join(game_folder_name);
                let file_path = game_data_folder.join(&link.name);
                fs::create_dir_all(&game_data_folder)?;
                match File::create(&file_path) {
                    Ok(mut file) => {
                        let content = response.bytes().await?;

                        if let Err(e) = file.write_all(&content) {
                            return Err(Io(e));
                        }

                        link.local_path = Some(file_path);
                    },
                    Err(e) => {
                        return Err(MessageError(format!("Error creating file \"{}\" : {:?}", &file_path.to_string_lossy(), e)));
                    }
                }

                Ok(())
            },
            Err(e) => {
                Err(GameResourceDownloadError(format!("Error downloading file \"{}\" : {:?}", &link.url, e)))
            }
        }
    }
    
    /// Return the folder name of this game based on its title.
    pub fn get_folder_name(&self) -> String {
        let mut folder_name = self.title.to_lowercase();
        folder_name.retain(|c| c.is_ascii_digit() || c.is_ascii_alphabetic());
        folder_name
    }
}