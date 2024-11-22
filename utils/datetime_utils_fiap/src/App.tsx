import { useEffect, useState } from "react";
import "./App.css";
import Clock from "./components/clock";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [dateTime, setDateTime] = useState("");

  useEffect(() => {
    const timer = setInterval(() => {
      invoke<string>("get_datetime").then(setDateTime);
    }, 1000);

    return () => clearInterval(timer);
  }, []);

  return (
    <main data-tauri-drag-region>
      <Clock 
        customText={'RM111111'}
        dateTime={dateTime}
      />
    </main>
  );
}

export default App;