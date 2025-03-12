import styles from "./Status.module.scss"
import ProgressBar from "../ProgressBar/ProgressBar.tsx";

interface Props{
    name:string,
    value:number
}
export default function Status({name, value}:Props){
    return(
        <div className={styles.container}>
            <span className={styles.container__name}>{name}:</span>
            <ProgressBar className={styles.container__bar} value={value} max={100} />
            <span className={styles.container__percent}>{value}%</span>
        </div>
    )
}