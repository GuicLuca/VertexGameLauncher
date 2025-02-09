import { invoke } from "@tauri-apps/api/core";
import { useGame } from './gameContext.tsx';

function DownloadButton() {
    const { selectedGame, downloadingGames, setDownloadingGames } = useGame();

    const isDownloaded = selectedGame.game_archive.link.local_path !== null;
    const isDownloadingNow = downloadingGames.has(selectedGame.id); // Vérifie si le jeu actuel est en cours de téléchargement

    const label = isDownloaded ? "Start" : isDownloadingNow ? "Downloading..." : "Download";

    const handleClick = async () => {
        if (isDownloaded) {
            invoke("launch", { game: selectedGame.id });
        } else {
            // Créer un nouveau Set pour forcer la mise à jour
            const newDownloadingGames = new Set(downloadingGames);
            newDownloadingGames.add(selectedGame.id);
            setDownloadingGames(newDownloadingGames);

            invoke("download", { game: selectedGame.id })
                .catch((error) => {
                    console.error('Error during download:', error);
                });
        }
    };

    return (
        <button
            className={`start-btn btn-strd ${isDownloadingNow ? "disabled" : ""}`}
            onClick={handleClick}
            disabled={isDownloadingNow} // Désactive le bouton si le jeu actuel est en cours de téléchargement
            style={{ opacity: isDownloadingNow ? 0.5 : 1, pointerEvents: isDownloadingNow ? "none" : "auto" }}
        >
            <div>{label}</div>
            <p>V{selectedGame.version}</p>
        </button>
    );
}

export default DownloadButton;
