import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useGame } from "./gameContext.tsx";

interface DownloadProgressProps {
  gameId: number; // On spécifie que gameId est un nombre
}

function downloadProgress({ gameId }: DownloadProgressProps) {

  const [downloadData, setDownloadData] = useState({
    downloaded: 0,
    file_size: 0,
    percentage: "0%",
    remaining_time: "N/A",
    speed: "0 MB/s",
    steps: "Starting",
  });
  const { selectedGame } = useGame(); // Récupérer le jeu sélectionné
  const [downloadingGameId, setDownloadingGameId] = useState<number | null>(null);

  useEffect(() => {
    // Écoute les événements pour ce jeu spécifique
    const eventName = `download_progress_${gameId}`;
    const unlisten = listen(eventName, (event) => {
      const data = event.payload as DownloadPayload;
      setDownloadData(data);
      console.log(data.steps)
      // Vérifie si le téléchargement est en cours et met à jour l'ID du jeu en téléchargement
      if (data.steps !== 'Complete') {
        setDownloadingGameId(gameId);
      } else {
        setDownloadingGameId(null);
      }
    });

    // Nettoyage lorsque le composant est démonté
    return () => {
      unlisten.then((fn) => fn());
    };
  }, [gameId]);

  // Vérifie si l'ID du jeu sélectionné est le même que celui en téléchargement
  if (selectedGame?.id !== downloadingGameId) return null;
  const progressPercentage = parseFloat(downloadData.percentage);

  return (
    <div className="download-progress">
      <h3>{downloadData.steps}</h3>
      <div className="progress-bar-container" style={{ marginBottom: "1rem" }}>
        <div
          className="progress-bar"
          style={{
            width: `${progressPercentage}%`,
            backgroundColor: "#4CAF50",
          }}
        ></div>
      </div>
      <div className="download-info">
        <p>
          <strong>Vitesse :</strong> <br />{downloadData.speed}
        </p>
        <p>
          <strong>Pourcentage :</strong> <br />{downloadData.percentage}
        </p>
        <p>
          <strong>Temps restant :</strong> <br />{downloadData.remaining_time}
        </p>
        <p>
          <strong>Size of the game :</strong> <br />{(downloadData.file_size / 1073741824).toFixed(2)} Go
        </p>
      </div>
    </div>
  );
};

export default downloadProgress;
