import styles from "../../App.module.scss";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";


type Response = {

}
export default function Ports() {
    const [ports, setPorts] = useState<string[]>([])

    useEffect(() =>{
        invoke('get_available_ports').then((res)=>{
            setPorts(res as string[])
        })
    },[])
    async function submitAction(){

    }


    return (
        <form className={styles.container}>
            <h1>Ports</h1>
                <input type="text"/>
            <select>{
                ports.map((p )=> <option>{p}</option>)
            }

            </select>
            <input type="submit"/>
        </form>
)
}