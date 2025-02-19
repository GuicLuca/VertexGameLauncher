# Vertex Launcher

___
## What is Vertex?

Vertex is a **free and open-source** game launcher designed for independent game developers who wish to distribute their 
games efficiently in a centralized platform.

Vertex requires an internet connection and operates based on a centrally hosted configuration file that can be stored
wherever you choose (Google Drive, Dropbox, Dedicated Server, etc.). Upon startup, Vertex reads the remote configuration 
file and synchronizes automatically. Thanks to this approach, there is no need to update Vertex to add new games.

### Technologies

- [Tauri framework](https://github.com/tauri-apps/tauri)
- **Backend:** Rust
- **FrontEnd:** React, Javascript/TypeScript

___

## How to use Vertex

#### 1 - Setup central configuration file

First things first, you need to create your central configuration file. By default, it should be a JSON file containing 
the key `games` which is an array of game object. Here is an example with on game:

````json5
// Example of central configuration file
{
    "games": [
        {
            "id": 1, // Numeric (UNIQUE): The identifier of the game
            "weight": 100, // Numeric: Used to order games in the list. Higher weight first
            "title": "...", // String: Main title of your game (used in the sidebar)
            "subtitle": "...", // String: Suffix title if needed
            "description": "...", // String: Small description of the game displayed in the game page
            "background_image": {
                "url": "...", // String: url to the image 
                "name": "...", // String: name of the file. It MUST contain the extension name
                "revision": 1 // Numeric: Version of this link. Each time you update the link you MUST increase this value
            },
            "navigation_icon": {
                "url": "...",
                "name": "...",
                "revision": 1
            },
            "download_link": {
                "link": {
                    "url": "...",
                    "name": "...",
                    "revision": 1
                },
                "need_extract": true, // Bool: true if the download_link is a zip archive that need to be extracted after download.
                "strip_top_level_folder": false, // Bool: If true, the extracting process will strip the top level folder of the archive
                "path_to_executable": "windows/my_game.exe" // String: Relative path to the executable once the extraction completed.
                
            },
            "version": "1.0.0", // String: Version of the game
            "platform": [
                "Windows"
            ], // Array<String>: Array containing all supported platform of your game
            "tags": [
                "Ubisoft contest 2024",
                "multiplayer",
                "school project",
                "Dream",
                "Unreal Engine"
            ] // Array<String>: Array containing tags that describe your game
        },
      // add as many games as you want in it :)
    ]
}
````

#### 2 - Setup Vertex environement
Once the previous phase has been achived, locate the environment variable file at [`vertex_launcher/src-tauri/src/env.rs`](vertex_launcher/src-tauri/src/env.rs), and setup environement values to adjuste the launcher behavior to your needs.

##### Mandatory setup
**Link the central configuration file to the vertex project**: 
Update the value of the `ONLINE_CONFIGURATION_FILE` variable with the public link to your central configuration file.
> Ensure alwayse that the link you set in the `ONLINE_CONFIGURATION_FILE` key is the **__PUBLIC__** link to your file, or the launcher won't be able to fetch it.

##### Optional setup
The [`env.rs`](vertex_launcher/src-tauri/src/env.rs) file contains around fifteen configuration variables which already have a default value designed to give general and pleasant behavior for the end user. We encourage you to go through them all to make sure they meet your needs (5 mins).

/!\ Localization is not currently supported by the launcher, so if the english in not your targeted language, you will need to update the function `generate_download_complete_message` to ensure notification's message are using the correct language.


##### 3 - Customize frontend views

The default frontend is made using React.js, TypeScript and Sass. The simplest way to adapt the frontend is to adapt the values ​​of constant style variables found at the top of the [`App.scss`](vertex_launcher/src/App.scss) file and then recompile it. If you don't know how to compile sass into css, [here is a short article](https://medium.com/@codingcarter/how-to-compile-scss-code-in-visual-studio-code-c8406fe54a18) introducing a very easy way to do it.

If the Vertex frontend does not meet your expectations, you might as well get rid of it and create your own frontend by connecting it to the Vertex backend. To do so, read the section [Lifecyle and events](#Lifecyle-and-events)

##### 4 - Build your Vertex

Once everything is ready, you can build your application following the tauri documentation page about distributing your app : [Distribute | Tauri](https://v2.tauri.app/distribute/).

That's it you've got your custom game launcher that you can share to everyone :)

___

## Lifecyle and events
*Every capitalized words represents const variable from [`env.rs`](vertex_launcher/src-tauri/src/env.rs).*
1. SplashScreen :
    - Init plugins
    - Setup logger and loading configuration
    - Setup system tray
    - Loading local game list
    - Fetching remote game list
    - Updating local game list
      - if needed: download new resources
    - => EVENT_INIT
    - close splashscreen and show main window
2. Main Window
    - Get game list > display > select first game
    - Download process:
        - Invoke to download(game_id)
        - => EVENT_DOWNLOAD_PROGRESS_{game_id} -> every UPDATE_FREQUENCY ms until download is complete
        - => EVENT_DOWNLOAD_COMPLETED_{game_id}
    - Check if game is donloaded: read if game->game_archive->link->local_path is valide
    - Closing app:
        - Closing main window will not stop the app (like steam or epic games)
        - Double clik on the system tray app icon will show/hide the main window
        - Right clik > Close to fully close the launcher.
        - App is allowing one unique instance only, so trying to launch the app multiple time will result to focussing the current open app instance. (and showing the main window if it's not currently visible)
3. App closing
    - Saving local game list to the file system (security to avoid inconsistency in data between two start)

___

## How to Contribute

If you don't want to code anything, you can bring valuable contribution by writing suggestion in the [issues page](https://github.com/GuicLuca/VertexGameLauncher/issues)! It help us a lot to keep focusing on what users need/want. :)

To contribute, you need to clone the repository and make your changes in a separate branch. Once completed, submit a pull request. The pull request will be reviewed before being integrated into the project. For a pull request to be accepted:
- The submitted code must be clear and well-commented.
- The code must adhere as closely as possible to development standards and best practices defined by Rust. [See Clippy lints](https://rust-lang.github.io/rust-clippy/master/index.html).
- The pull request must have a clear title and description.
- Only modify what is stated in the pull request, avoiding multiple features in a single PR.
- The changes must have a positive impact on the project (new features, bug fixes, optimizations, etc.).


___
## Thanks

Special thanks to [@TX-Mat](https://github.com/TxMat) and [@Marco-Vassal](https://github.com/Marco-Vassal) for their valuable help on starting this project! I hope this will be usefull and that you will enjoy it!


___

## License
This project is distributed under the [GNU Affero General Public License](LICENSE). 

It also utilizes the [Tauri framework](https://github.com/tauri-apps/tauri), which is distributed under the [MIT License](LICENSE_Tauri).  
Copyright (c) 2020-2023 Tauri Programme within The Commons Conservancy.
