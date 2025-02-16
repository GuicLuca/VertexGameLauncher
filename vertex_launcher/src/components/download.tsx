import {invoke} from "@tauri-apps/api/core";
import {useGame} from './gameContext.tsx';

function DownloadButton() {
    const {selectedGame, downloadingGames, setDownloadingGames} = useGame();

    const isDownloaded = selectedGame.game_archive.link.local_path !== null;
    const isDownloadingNow = downloadingGames.has(selectedGame.id); // Check if the game is currently being downloaded

    const label = isDownloaded ? "Start" : isDownloadingNow ? "Downloading..." : "Download";

    const handleClick = async () => {
        if (isDownloaded) {
            invoke("launch", {game: selectedGame.id});
        } else {
            // Force the update of the component by creating a new Set (changing the state)
            const newDownloadingGames = new Set(downloadingGames);
            newDownloadingGames.add(selectedGame.id);
            setDownloadingGames(newDownloadingGames);

            invoke("download", {game: selectedGame.id})
                .catch((error) => {
                    console.error('Error during download:', error);
                });
        }
    };


    return (
        <button
            className={`start-btn btn-strd ${isDownloadingNow ? "disabled" : ""}`}
            onClick={handleClick}
            disabled={isDownloadingNow} // disable the button if the game is currently being downloaded
            style={{opacity: isDownloadingNow ? 0.5 : 1, pointerEvents: isDownloadingNow ? "none" : "auto"}}
        >
            <div>{label}</div>
            <p>V{selectedGame.version}</p>
        </button>
    );
}

export default DownloadButton;
