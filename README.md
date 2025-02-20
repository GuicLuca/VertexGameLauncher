# Vertex Launcher

___
## What is Vertex?

Vertex is a **free and open-source** game launcher designed for independent game developers who wish to distribute their games efficiently on a centralized platform.

Vertex requires an internet connection and operates based on a centrally hosted configuration file, which can be stored wherever you choose (Google Drive, Dropbox, a dedicated server, etc.). Upon startup, Vertex reads the remote configuration file and synchronizes automatically. Thanks to this approach, there is no need to update Vertex to add new games.

### Technologies

- [Tauri framework](https://github.com/tauri-apps/tauri)
- **Backend:** Rust
- **Frontend:** React, JavaScript/TypeScript

___

## How to Use Vertex

#### 1 - Setup Central Configuration File

First things first, you need to create your central configuration file. By default, it should be a JSON file containing the key `games`, which is an array of game objects. Here is an example with one game:

```json5
// Example of central configuration file
{
    "games": [
        {
            "id": 1, // Numeric (UNIQUE): The identifier of the game
            "weight": 100, // Numeric: Used to order games in the list. Higher weight comes first
            "title": "...", // String: Main title of your game (used in the sidebar)
            "subtitle": "...", // String: Suffix title if needed
            "description": "...", // String: Brief description of the game displayed on the game page
            "background_image": {
                "url": "...", // String: URL to the image 
                "name": "...", // String: Name of the file. It MUST contain the file extension
                "revision": 1 // Numeric: Version of this link. Each time you update the link, you MUST increase this value
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
                "need_extract": true, // Bool: true if the download link is a zip archive that needs to be extracted after download.
                "strip_top_level_folder": false, // Bool: If true, the extraction process will strip the top-level folder of the archive
                "path_to_executable": "windows/my_game.exe" // String: Relative path to the executable once the extraction is completed.
            },
            "version": "1.0.0", // String: Version of the game
            "platform": [
                "Windows"
            ], // Array<String>: Array containing all supported platforms for your game
            "tags": [
                "Ubisoft contest 2024",
                "multiplayer",
                "school project",
                "Dream",
                "Unreal Engine"
            ] // Array<String>: Array containing tags that describe your game
        }
        // Add as many games as you want :)
    ]
}
```

#### 2 - Setup Vertex Environment
Once the previous phase has been achieved, locate the environment variable file at [`vertex_launcher/src-tauri/src/env.rs`](vertex_launcher/src-tauri/src/env.rs), and set up environment values to adjust the launcher's behavior to your needs.

##### Mandatory Setup
Link the central configuration file to the Vertex project:
Update the value of the `ONLINE_CONFIGURATION_FILE` variable with the public link to your central configuration file.

Always ensure that the link set in the `ONLINE_CONFIGURATION_FILE` variable is the PUBLIC link to your file, or the launcher won't be able to fetch it.

##### Optional Setup
The [`env.rs`](vertex_launcher/src-tauri/src/env.rs) file contains around fifteen configuration variables that already have default values designed to provide a generally pleasant experience for the end user. We encourage you to review them to ensure they meet your needs (approximately 5 minutes).

**Note**: Localization is not currently supported by the launcher. If English is not your target language, you will need to update the function `generate_download_complete_message` to ensure that the notification messages use the correct language.

#### 3 - Customize Frontend Views
The default frontend is built using React.js, TypeScript, and Sass. The simplest way to customize the frontend is to adjust the constant style variables found at the top of the [`App.scss`](vertex_launcher/src/App.scss) file and then recompile it. If you don't know how to compile Sass into CSS, here is a short article that introduces an easy method to do so.

If the Vertex frontend does not meet your expectations, you can remove it and create your own frontend by connecting it to the Vertex backend. To do so, read the section [Lifecycle and Events](#Lifecycle-and-Events).

#### 4 - Build Your Vertex
Once everything is ready, you can build your application by following the Tauri documentation on distributing your app: [Distribute | Tauri](https://v2.tauri.app/distribute/).

That's it! You've got your custom game launcher that you can share with everyone!

___

## Lifecycle and Events
Every capitalized word represents a constant variable from [`env.rs`](vertex_launcher/src-tauri/src/env.rs).

1. SplashScreen:
- Initialize plugins
- Set up the logger and load configuration
- Set up the system tray
- Load the local game list
- Fetch the remote game list
- Update the local game list
   -If needed: download new resources
- => EVENT_INIT
- Close the splash screen and show the main window

2. Main Window:
- Retrieve the game list, display it, and select the first game
- Download Process:
   - Invoke the download function (download(game_id))
   - => EVENT_DOWNLOAD_PROGRESS_{game_id} — emitted every UPDATE_FREQUENCY milliseconds until the download is complete
   - => EVENT_DOWNLOAD_COMPLETED_{game_id}
- Check if the game is downloaded by verifying that game->game_archive->link->local_path is valid

3. Closing the App:
- Closing the main window will not stop the app (similar to Steam or Epic Games)
- Double-click the system tray icon to show/hide the main window
- Right-click > Close to fully exit the launcher
- The app allows only a single instance, so trying to launch the app multiple times will result in focusing the currently open instance (and showing the main window if it's not visible)

4. App Closing:
- Save the local game list to the file system (to ensure data consistency between launches)
- How to Contribute
- If you don't want to code anything, you can make a valuable contribution by writing suggestions on the issues page! It helps us a lot to focus on what users need and want. :)

___

## How to contribute

To contribute, clone the repository and make your changes on a separate branch. Once completed, submit a pull request. The pull request will be reviewed before being integrated into the project. For a pull request to be accepted:

The submitted code must be clear and well-commented.
The code must adhere as closely as possible to the development standards and best practices defined by Rust. See Clippy lints.
The pull request must have a clear title and description.
Only modify what is stated in the pull request, avoiding multiple features in a single PR.
The changes must have a positive impact on the project (new features, bug fixes, optimizations, etc.).

___ 

## Thanks
Special thanks to [@TX-Mat](https://github.com/TxMat) and [@Marco-Vassal](https://github.com/Marco-Vassal) for their valuable help in starting this project! I hope this will be useful and that you will enjoy it!

___ 

## License
This project is distributed under the GNU Affero General Public License.

It also utilizes the Tauri framework, which is distributed under the MIT License.
© 2020-2023 Tauri Programme within The Commons Conservancy.
