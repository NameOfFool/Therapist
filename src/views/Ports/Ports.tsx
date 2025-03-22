import styles from "./Ports.module.scss";
import {FormEvent, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";


type Request = {
    port_name:string
}
export default function Ports() {
    const [ports, setPorts] = useState<string[]>([])
    const [selectedPort, setSelectedPort] = useState<string>()

    useEffect(() =>{
        invoke('get_available_ports').then((res)=>{
            setPorts(res as string[])
        })
    },[])
    async function submitAction(e: FormEvent<HTMLFormElement>){
        e.preventDefault()
        invoke<Request>('open_port', {port_name:selectedPort } ).then((res)=>console.log(res)).catch((e) => console.error(e))
    }


    return (
        <div className={styles.container} >
            <h1>Ports</h1>
            <form className={styles.container__form} onSubmit={submitAction}>
            <select value={selectedPort} onChange={(e) => setSelectedPort(e.target.value)}>{
                ports.map((p , index)=> <option key={index} value={p}>{p}</option>)
            }

            </select>
            <input type="submit" className={styles.container__button}/>
            </form>
        </div>
)
}