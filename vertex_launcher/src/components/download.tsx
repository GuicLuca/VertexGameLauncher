// import React from 'react';
import { invoke } from "@tauri-apps/api/core";
import { useGame } from './gameContext.tsx';
// import { info } from 'tauri-plugin-log-api';

function download() {
    const { selectedGame } = useGame();
    let isDownloaded = false;

    const checkDownloadStatus = () => {
        isDownloaded = selectedGame.game_archive.link.local_path !== null;
    };

    checkDownloadStatus();

    const label = isDownloaded ? 'Start' : 'Download';


    const handleClick = async () => {
        if (isDownloaded) {
            // start the game
            invoke("launch", { game: selectedGame.id })
        } else {
            // download the game
            invoke("download", { game: selectedGame.id}
            ).catch(
                (error) => {
                    console.error('Error during download :', error);
                }
            );
        }
    }

    return (
        <div className='start-btn btn-strd'
            onClick={() => handleClick()}
        >
                <div>{label}</div>
                <p>V{selectedGame.version}</p>
            </div>
      )
}

export default download