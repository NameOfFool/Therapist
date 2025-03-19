import styles from "./Status.module.scss"
import  {ApexOptions} from 'apexcharts'
import ReactApexChart from 'react-apexcharts'

interface Props{
    options:ApexOptions,
    series:ApexAxisChartSeries|ApexNonAxisChartSeries,
    data:string
}
export default function Status({options, series, data}:Props){
    return(
        <div className={styles.container}>
            <div className={styles.container__bar}>
                <ReactApexChart options={options} series={series} type="line" height={350} /></div>
            <span className={styles.container__percent}>{data}</span>
        </div>
    )
}