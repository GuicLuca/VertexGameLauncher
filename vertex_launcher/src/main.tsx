import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { once } from "@tauri-apps/api/event";

// ========= Logging config =========

// Import the log plugin from the Tauri API
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

/**
 * This function forwards console messages to the Tauri logger.
 * see : https://v2.tauri.app/plugin/logging/#logging
 * @param fnName LogLevel
 * @param logger Logger function
 */
export function forwardConsole(
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

export function getFormatedBytes(bytes: number): string {
  const sizes = ["B", "KiB", "MiB", "GiB", "TiB"];
  if (bytes === 0) return "0B";
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return parseFloat((bytes / Math.pow(1024, i)).toFixed(2)) + " " + sizes[i];
}



// Listen for the app to be initialized
once("app_initialized", () => {
  info("[Frontend] App initialized event received.");
  // test_backend_functions();

  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
});




