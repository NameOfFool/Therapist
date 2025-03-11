import "./App.scss";
import Status from "./componenst/Status/Status.tsx";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";
interface SystemStatus {
    cpuUsage: number,
    ramUsage: number,
    ramTotal: number
}

function App() {
    async function UpdateStatus(){
        setTimeout(async () =>{
            await invoke("get_status").then((response) => {
                setSystem(response as SystemStatus)
            })
        })
    }
    const [system, setSystem] = useState<SystemStatus>({
        cpuUsage:0,
        ramUsage:0,
        ramTotal:0
    })
    useEffect(() => {
        setTimeout(async () =>UpdateStatus(), 2000)
    }, []);
  return (
    <main className="container">
        <span id="test"></span>
        <Status name={"CPU"} value={system.cpuUsage}/>
        <button content={"123"} onClick={() => {
            setTimeout(async () => {
            await invoke("get_status");
        }, 1000);
        }} />
    </main>
  );
}

export default App;
