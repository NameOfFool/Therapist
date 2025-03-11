import styles from './ProgressBar.module.scss'
interface ProgressBarProps{
    value:number,
    max:number
}
export default function ProgressBar({ value, max}:ProgressBarProps) {
    return (
        <div className={styles.progressBar}>
            <div className={styles.progressBar_completed} style={{width: `${(value / max) * 100}%`}}>
            </div>
        </div>
    );
}