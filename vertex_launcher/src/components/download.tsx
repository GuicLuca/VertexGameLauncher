import { invoke } from "@tauri-apps/api/core";
import { useGame } from './gameContext';
import { useEffect } from 'react';

function DownloadButton() {
    const { selectedGame, downloadingGames, setDownloadingGames } = useGame();

    // Check if game is downloaded by verifying local_path exists
    const isDownloaded = selectedGame?.game_archive?.link?.local_path !== null;
    // Check if game is currently being downloaded
    const isDownloadingNow = downloadingGames.has(selectedGame?.id as number);

    // Set button label based on game state
    const label = isDownloaded ? "Start" : isDownloadingNow ? "Downloading..." : "Download";

    const handleClick = async () => {
        if (!selectedGame) return;

        if (isDownloaded) {
            // If game is downloaded, launch it
            try {
                await invoke("launch", { game: selectedGame.id });
            } catch (error) {
                console.error('Error launching game:', error);
            }
        } else {
            // If game isn't downloaded, start download
            try {
                // Add game to downloading set
                const newDownloadingGames = new Set(downloadingGames);
                newDownloadingGames.add(selectedGame.id);
                setDownloadingGames(newDownloadingGames);

                // Start download
                await invoke("download", { game: selectedGame.id });
            } catch (error) {
                console.error('Error during download:', error);
                // Remove from downloading games in case of error
                const errorDownloadingGames = new Set(downloadingGames);
                errorDownloadingGames.delete(selectedGame.id);
                setDownloadingGames(errorDownloadingGames);
            }
        }
    };

    // Cleanup effect for when component unmounts
    useEffect(() => {
        return () => {
            if (isDownloadingNow && selectedGame) {
                const cleanupDownloadingGames = new Set(downloadingGames);
                cleanupDownloadingGames.delete(selectedGame.id);
                setDownloadingGames(cleanupDownloadingGames);
            }
        };
    }, []);

    // If no game is selected, don't render the button
    if (!selectedGame) {
        return null;
    }

    return (
        <button
            className={`start-btn btn-strd ${isDownloadingNow ? "disabled" : ""}`}
            onClick={handleClick}
            disabled={isDownloadingNow}
            style={{
                opacity: isDownloadingNow ? 0.5 : 1,
                pointerEvents: isDownloadingNow ? "none" : "auto"
            }}
        >
            <div>{label}</div>
            <p>V{selectedGame.version}</p>
        </button>
    );
}

export default DownloadButton;