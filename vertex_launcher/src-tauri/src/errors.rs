pub type Result<T> = std::result::Result<T, Verror>;
#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum Verror {
    // BASIC ERRORS
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    ZipError(#[from] zip_extract::ZipExtractError),
    
    #[error(transparent)]
    TauriError(#[from] tauri::Error),
    
    #[error(transparent)]
    Error(#[from] Box<dyn std::error::Error>),
    
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    
    #[error(transparent)]
    ReqwestError(#[from] tauri_plugin_http::reqwest::Error),
    
    #[error("{0}")]
    MessageError(String),
    
    // GAME ERRORS
    #[error("Invalid JSON data for constructing a game. Got json:\n {0}")]
    GameConstructionError(String),
    
    #[error("An error occurred while downloading resources for the game {0}")]
    GameResourceDownloadError(String),
    
    #[error("An error occurred while extracting resources for the game {0}")]
    GameResourceExtractionError(String),
    
    #[error("An error occurred while launching the game {0}")]
    GameLaunchError(String),
    
    #[error("An error occurred while fetching the game {0} from the local list.")]
    GameListFetchError(String),

    // STORE ERRORS
    #[error("An error occurred while fetching the store at {0}")]
    StoreAccessError(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Verror {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
