import { useState } from "react";
import reactLogo from "./assets/react.svg";
// import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import GameList from "./components/gameList.jsx";
import GamePage from "./components/gamePage.jsx";

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <div className="launcher">
     <GameList/>
     <GamePage/>
    </div>
  );
}

export default App;
