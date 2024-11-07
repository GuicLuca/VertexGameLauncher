import React, { useEffect, useState } from 'react';
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { GameProvider, useGame } from './components/gameContext.jsx';
import GameList from "./components/gameList.jsx";
import GamePage from "./components/gamePage.jsx";
import { info } from "tauri-plugin-log-api";

function App() {
  const [games, setGames] = useState([]);
  const { setSelectedGame } = useGame(); // Utiliser le setter de selectedGame du contexte

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }
  
  useEffect(() => {
    // Charger le JSON local
    fetch('/Games.json')
      .then((response) => response.json())
      .then((data) => {
        setGames(data.games);
        // Sélectionner le premier jeu comme jeu par défaut si la liste n'est pas vide
        if (data.games.length > 0) {
          setSelectedGame(data.games[0]);
        }
      })
      .catch((error) => console.error('Erreur lors du chargement du JSON :', error));
  }, [setSelectedGame]);

  return (
    <div className="launcher App">
      <GameList games={games} /> {/* Passer les jeux à GameList */}
      <GamePage />
    </div>
  );
}

export default function AppWrapper() {
  return (
    <GameProvider>
      <App />
    </GameProvider>
  );
}
