import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useGame } from "./gameContext.tsx";
import { getFormatedBytes } from "../main.tsx";

interface DownloadProgressProps {
  gameId: number; // Specify the game id is a number for the rest of the widget
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
  const { selectedGame } = useGame(); // Fetch the selected game from the context
  const [downloadingGameId, setDownloadingGameId] = useState<number | null>(null);

  useEffect(() => {
    // Make this component listen to the download progress of his game only
    const eventName = `download_progress_${gameId}`;
    const unlisten = listen(eventName, (event) => {
      const data = event.payload as DownloadPayload;
      setDownloadData(data);
      // Check if the download is in progress and set the downloadingGameId
      if (data.steps !== 'Complete') {
        setDownloadingGameId(gameId);
      } else {
        setDownloadingGameId(null);
      }
    });

    
    return () => {
      // Clean up the listener once the component is unmounted
      unlisten.then((fn) => fn());
    };
  }, [gameId]);

  // Check if the selected game is the one being downloaded to display the progress widget only on the targeted game
  if (selectedGame?.id !== downloadingGameId) return null;
  const progressPercentage = parseFloat(downloadData.percentage);

  return (
    <div className="download-progress">
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
          <strong>Speed:</strong> {downloadData.speed}
        </p>
        <p>
          <strong>Progress:</strong> {downloadData.percentage}
        </p>
        <p>
          <strong>Remainng time:</strong> {downloadData.remaining_time}
        </p>
        <p>
          <strong>Totale size:</strong> {getFormatedBytes(downloadData.file_size)}
        </p>
      </div>
      <h3>{downloadData.steps}</h3>
    </div>
  );
};

export default downloadProgress;
