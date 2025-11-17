import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState } from "react";
import "./App.css";
import reactLogo from "./assets/react.svg";
import { initTray } from "./tray";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const trayInitialized = useRef(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!trayInitialized.current) {
      trayInitialized.current = true;
      initTray().catch((error) => {
        console.error(error);
        setError(
          `Failed to initialize tray: ${error} \n\n${
            error instanceof Error ? error.stack : ""
          }`
        );
      });
    }
  }, []);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>
      <pre>{error}</pre>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
