import React, { createContext, useContext, useState } from 'react';

// Context creation
const GameContext = createContext();

// Context provider to wrap the application
export const GameProvider = ({ children }) => {
  const [selectedGame, setSelectedGame] = useState(null);

  return (
    <GameContext.Provider value={{ selectedGame, setSelectedGame }}>
      {children}
    </GameContext.Provider>
  );
};

// Hook to use easly the context
export const useGame = () => useContext(GameContext);
