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
  const { setSelectedGame } = useGame(); // use the setter of selectedGame from the contexte

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }
  
  useEffect(() => {
    // Load the local Json
    fetch('/Games.json')
      .then((response) => response.json())
      .then((data) => {
        setGames(data.games);
        // Select the first game inside the json by default
        if (data.games.length > 0) {
          setSelectedGame(data.games[0]);
        }
      })
      .catch((error) => console.error('Erreur lors du chargement du JSON :', error));
  }, [setSelectedGame]);

  return (
    <div className="launcher App">
      <GameList games={games} /> {/* Give the game list to GameList */}
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
