import styles from "./Status.module.scss"
import ProgressBar from "../ProgressBar/ProgressBar.tsx";
import {useEffect, useState} from "react";
import ApexCharts, {ApexOptions} from 'apexcharts'
import ReactApexChart from 'react-apexcharts'

interface Props{
    options:ApexOptions,
    series:ApexAxisChartSeries|ApexNonAxisChartSeries,
    value:number
}
export default function Status({options, series, value}:Props){
    return(
        <div className={styles.container}>
            <div className={styles.container__bar}>
                <ReactApexChart options={options} series={series} type="line" height={350} /></div>
            <span className={styles.container__percent}>{value}%</span>
        </div>
    )
}