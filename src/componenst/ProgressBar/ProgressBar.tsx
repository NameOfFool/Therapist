import styles from './ProgressBar.module.scss'
interface ProgressBarProps{
    value:number,
    max:number,
    className:string
}
export default function ProgressBar({ value, max, className}:ProgressBarProps) {
    return (
        <div className={`${styles.progressBar} ${className}`}>
            <div className={styles.progressBar_completed} style={{width: `${(value / max) * 100}%`}}>
            </div>
        </div>
    );
}