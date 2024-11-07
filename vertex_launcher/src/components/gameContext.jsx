import React, { createContext, useContext, useState } from 'react';

// Création du contexte
const GameContext = createContext();

// Fournisseur de contexte pour envelopper l'application
export const GameProvider = ({ children }) => {
  const [selectedGame, setSelectedGame] = useState(null);

  return (
    <GameContext.Provider value={{ selectedGame, setSelectedGame }}>
      {children}
    </GameContext.Provider>
  );
};

// Hook pour utiliser le contexte facilement
export const useGame = () => useContext(GameContext);
