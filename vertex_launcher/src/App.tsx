import { useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { GameProvider, useGame } from './components/gameContext.tsx';
import GameList from "./components/gameList.tsx";
import GamePage from "./components/gamePage.tsx";
import { info } from "tauri-plugin-log-api";
import { forwardConsole } from "./main.tsx"
import { listen } from '@tauri-apps/api/event';
import Game from "./models/game.tsx";

function App() {
  const [games, setGames] = useState([] as Game[]); // Create a state for the game list
  const { selectedGame, setSelectedGame } = useGame(); // use the setter of selectedGame from the contexte
  forwardConsole('info', info);
  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // };

  listen("game_list_updated", (event) => {
    console.log(JSON.stringify(event.payload));
    // update the game list with the new one
    setGames(JSON.parse(event.payload as string));
    // select the game that was selected before
    let updated_selected_game = games.find((game) => {
      return game.id === selectedGame.id;
    });

    if (updated_selected_game === undefined) {
      updated_selected_game = games[0];
    }

    setSelectedGame(updated_selected_game);
  });
  
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
