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

## Quick start

First things first, you need to create your central configuration file. By default, it should be a JSON file containing 
the key `games` which is an array of game object. Here is an example with on game:

````json5
// Example of central configuration file
{
    "games": [
        {
            "id": 1, // nNumeric (UNIQUE): The identifier of the game
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

Once achieved, locate the environment variable file at `vertex_launcher/src-tauri/src/env.rs`, 
and update the value of the `ONLINE_CONFIGURATION_FILE` variable with the public link to your central configuration file.

___

## How to Contribute

To contribute, you need to clone the repository and make your changes in a separate branch. Once completed, submit a pull request. The pull request will be reviewed before being integrated into the project.

For a pull request to be accepted:
- The submitted code must be clear and well-commented.
- The code must adhere as closely as possible to development standards and best practices defined by Rust. [See Clippy lints](https://rust-lang.github.io/rust-clippy/master/index.html).
- The pull request must have a clear title and description.
- Only modify what is stated in the pull request, avoiding multiple features in a single PR.
- The changes must have a positive impact on the project (new features, bug fixes, optimizations, etc.).



___

## License
This project is distributed under the [GNU Affero General Public License](LICENSE). 

It also utilizes the [Tauri framework](https://github.com/tauri-apps/tauri), which is distributed under the [MIT License](LICENSE_Tauri).  
Copyright (c) 2020-2023 Tauri Programme within The Commons Conservancy.
