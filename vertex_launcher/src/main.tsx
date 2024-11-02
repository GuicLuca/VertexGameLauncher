import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

// ========= Logging config =========

// Import the log plugin from the Tauri API
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

/**
 * This function forwards console messages to the Tauri logger.
 * see : https://v2.tauri.app/plugin/logging/#logging
 * @param fnName LogLevel
 * @param logger Logger function
 */
function forwardConsole(
  fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
  logger: (message: string) => Promise<void>
) {
  const original = console[fnName];
  console[fnName] = (message) => {
    original(message);
    logger(message);
  };
}

forwardConsole('log', trace);
forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);

//~ End of logging config ==========

// ========= Backend_Testing =========
import { invoke } from "@tauri-apps/api/core";
import { listen, once } from "@tauri-apps/api/event";
function test_backend_functions() {

  invoke('get_game_list').then((result) => {
    const obj = JSON.parse(result as string);
    info("Game list received: " + obj);
  }).catch((err: string) => {
    error(err);
  });

  // invoke("get_launcher_version").then((result) => {
  //   info("Launcher version received: " + result);
  // });
}

//~ End of Backend_Testing ==========

// Listen for the app to be initialized
once("app_initialized", () => {
  info("App initialized");
  test_backend_functions();




  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
});





