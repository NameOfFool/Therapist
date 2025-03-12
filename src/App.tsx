import styles from "./App.module.scss";
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
        await invoke("get_status")
        listen("status", (event) =>{
            setSystem(event.payload as SystemStatus)
        })
    }
    const [system, setSystem] = useState<SystemStatus>({
        cpuUsage:0,
        ramUsage:0,
        ramTotal:0
    })
    useEffect(() => {
        UpdateStatus()
    }, []);
  return (
    <main className={styles.container}>
        <h1>Overview</h1>
        <div className={styles.container__stats}>
        <Status name={"CPU"} value={Math.round(system.cpuUsage)}/>
        <Status name={"RAM"} value={
            system.ramTotal==0?0:Math.round(100.0 * system.ramUsage/system.ramTotal)}/>
        </div>
    </main>
  );
}

export default App;
