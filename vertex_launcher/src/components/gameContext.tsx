import { createContext, useContext, useState, ReactNode } from 'react';
import Game from '../models/game';


interface GameContextType {
  selectedGame: Game;
  setSelectedGame: (game: Game) => void;
}

const GameContext = createContext<GameContextType | undefined>(undefined);

export const GameProvider = ({ children }: { children: ReactNode }) => {
  const [selectedGame, setSelectedGame] = useState<any>(null);

  return (
    <GameContext.Provider value={{ selectedGame, setSelectedGame }}>
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
