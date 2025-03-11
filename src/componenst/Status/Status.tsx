import styles from "./Status.module.scss"
import ProgressBar from "../ProgressBar/ProgressBar.tsx";

interface Props{
    name:string,
    value:number
}
export default function Status({name, value}:Props){
    return(
        <div className={styles.container}>
            <span>{name}:</span>
            <ProgressBar value={value} max={100} />
            <span>{value}%</span>
        </div>
    )
}