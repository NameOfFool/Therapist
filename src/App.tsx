import "./App.scss";
import Status from "./componenst/Status/Status.tsx";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
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
        setInterval(async () =>UpdateStatus(), 2000)
    }, []);
  return (
    <main className="container">
        <Status name={"CPU"} value={Math.round(system.cpuUsage)}/>
        <Status name={"RAM"} value={Math.round(100.0 * system.ramUsage/system.ramTotal)}/>
    </main>
  );
}

export default App;
