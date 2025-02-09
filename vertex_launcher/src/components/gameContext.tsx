import { createContext, useContext, useState, ReactNode, useEffect } from 'react';
import { listen } from "@tauri-apps/api/event";
import Game from '../models/game';

interface GameContextType {
  selectedGame: Game;
  setSelectedGame: (game: Game) => void;
  downloadingGames: Set<number>;
  setDownloadingGames: (games: Set<number>) => void;
}

const GameContext = createContext<GameContextType | undefined>(undefined);

export const GameProvider = ({ children }: { children: ReactNode }) => {
  const [selectedGame, setSelectedGame] = useState<any>(null)
  const [downloadingGames, setDownloadingGames] = useState<Set<number>>(new Set());
  const mySelectedGame = selectedGame;
  useEffect(() => {
    const unlistenCallbacks: Promise<() => void>[] = [];

    downloadingGames.forEach((gameId) => {
      const eventName = `download_completed_${gameId}`;
      const unlisten = listen(eventName, (event) => {
        setSelectedGame(mySelectedGame)
        setDownloadingGames(prevDownloadingGames => {
          const newDownloadingGames = new Set(prevDownloadingGames);
          newDownloadingGames.delete(gameId);
          return newDownloadingGames;
        });
      });

      unlistenCallbacks.push(unlisten);
    });
    
    return () => {
      unlistenCallbacks.forEach(unlistenPromise => unlistenPromise.then(fn => fn()));
    };
  }, [downloadingGames]); // On relance l'effet à chaque changement de `downloadingGames`
  return (
    <GameContext.Provider value={{ selectedGame, setSelectedGame, downloadingGames, setDownloadingGames }}>
      {children}
    </GameContext.Provider>
  );
};

export const useGame = () => {
  const context = useContext(GameContext);
  if (!context) {
    throw new Error('useGame must be used within a GameProvider');
  }
  return context;
};
