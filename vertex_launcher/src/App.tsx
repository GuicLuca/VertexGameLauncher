import { useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { GameProvider, useGame } from './components/gameContext.tsx';
import GameList from "./components/gameList.tsx";
import GamePage from "./components/gamePage.tsx";
import { info } from "tauri-plugin-log-api";
import { forwardConsole } from "./main.tsx"

function App() {
  const [games, setGames] = useState([]);
  const { setSelectedGame } = useGame(); // use the setter of selectedGame from the contexte

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }
  
forwardConsole('info', info);
  
  useEffect(() => {
    // Load the local Json
    invoke('get_game_list')
      .then((response) => {
        let data = JSON.parse(response as string);
        setGames(data);
        if (data.length > 0) {
          setSelectedGame(data[0]);
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
