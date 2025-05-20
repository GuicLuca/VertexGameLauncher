import {createContext, useContext, useState, ReactNode, useEffect} from 'react';
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";
import Game from '../models/game';

interface GameContextType {
    selectedGame: Game | null;
    setSelectedGame: (game: Game | null) => void;
    downloadingGames: Set<number>;
    setDownloadingGames: (games: Set<number>) => void;
}

const GameContext = createContext<GameContextType | undefined>(undefined);

export const GameProvider = ({children}: { children: ReactNode }) => {
    const [selectedGame, setSelectedGame] = useState<Game | null>(null);
    const [downloadingGames, setDownloadingGames] = useState<Set<number>>(new Set());

    useEffect(() => {
    const unlistenCallbacks: Promise<() => void>[] = [];

    downloadingGames.forEach((gameId) => {
        const eventName = `download_completed_${gameId}`;
        const unlisten = listen(eventName, async (_event) => {
            try {
                // Fetch the updated game data
                const updatedGameJson = await invoke<string>("get_game", { game: gameId });
                const updatedGame = JSON.parse(updatedGameJson);

                // Update the selected game if it matches
                if (selectedGame && selectedGame.id === gameId) {
                    setSelectedGame(updatedGame);
                }

                // Remove from downloading games
                setDownloadingGames(prevDownloadingGames => {
                    const newDownloadingGames = new Set(prevDownloadingGames);
                    newDownloadingGames.delete(gameId);
                    return newDownloadingGames;
                });
            } catch (error) {
                console.error('Error updating game after download:', error);
            }
        });

        unlistenCallbacks.push(unlisten);
    });

    return () => {
        unlistenCallbacks.forEach(unlistenPromise => unlistenPromise.then(fn => fn()));
    };
}, [downloadingGames, selectedGame]); // Added selectedGame to dependencies

    return (
        <GameContext.Provider value={{selectedGame, setSelectedGame, downloadingGames, setDownloadingGames}}>
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