import styles from "../../App.module.scss";
import Status from "../../components/Status/Status.tsx";
import {ApexOptions} from "apexcharts";
import ApexCharts from "apexcharts";
import {listen} from "@tauri-apps/api/event";
import {useEffect, useState} from "react";

interface SystemStatus {
    cpuUsage: number,
    ramUsage: number,
    ramTotal: number
}

export default function Overview() {

    async function UpdateStatus(){
        listen("status", (event) =>{
            setSystem((event.payload as SystemStatus))
        })
    }
    interface Point{
        x:Date,
        y:number
    }
    const [system, setSystem] = useState<SystemStatus>({
        cpuUsage:0,
        ramUsage:0,
        ramTotal:0
    })
    const [cpuData, setCpuData] = useState<Point[]>([])
    const [ramData, setRamData] = useState<Point[]>([])
    useEffect(() => {
        UpdateStatus().then( () => "listen begun")
    }, []);
    useEffect(() => {
        setCpuData([...cpuData, {x:new Date(Date.now()), y:Math.round(system.cpuUsage)}])
        setRamData([...ramData, {x:new Date(Date.now()), y:Math.round(system.ramTotal==0?0:100.0 * system.ramUsage/system.ramTotal)}])
        ApexCharts.exec('cpu', 'updateSeries', [{
            data: [...cpuData]
        }])
        ApexCharts.exec('ram', 'updateSeries', [{
            data: [...ramData]
        }])
    }, [system])
    const [state] =  useState({

        cpuSeries: [{
            name:'CPU',
            data: [...cpuData]
        }],
        ramSeries:[{
            name:'RAM',
            data:[...ramData]
        }],
        cpuOptions: {
            chart: {
                id: 'cpu',
                height: 150,
                type: 'line',
                animations: {
                    enabled: true,
                    easing: 'linear',
                    dynamicAnimation: {
                        speed: 1000
                    }
                },
                toolbar: {
                    show: false
                },
                zoom: {
                    enabled: false
                }
            },
            colors: ['#546E7A'],
            dataLabels: {
                enabled: false
            },
            stroke: {
                curve: 'smooth'
            },
            title: {
                text: 'CPU',
                align: 'left'
            },
            markers: {
                size: 0
            },
            xaxis:{
                type: 'datetime',
                range: 86400,
            },
            yaxis: {
                max: 100,
                min:0
            },
            legend: {
                show: false
            },

        },
        ramOptions: {
            chart: {
                id: 'ram',
                height: 150,
                type: 'line',
                animations: {
                    enabled: true,
                    easing: 'linear',
                    dynamicAnimation: {
                        speed: 1000
                    }
                },
                toolbar: {
                    show: false
                },
                zoom: {
                    enabled: false
                }
            },
            colors: ['#00E396'],
            dataLabels: {
                enabled: false
            },
            stroke: {
                curve: 'smooth'
            },
            title: {
                text: 'RAM',
                align: 'left'
            },
            markers: {
                size: 0
            },
            xaxis: {
                type: 'datetime',
                range: 86400,
            },
            yaxis: {
                max: 100,
                min:0
            },
            legend: {
                show: false
            },
        }
    });
    function bytesToGB(bytes:number){
        return bytes/Math.pow(1024,3)
    }
    return (
        <div>
            <h1>Overview</h1>
            <div className={styles.container__stats} id="cpu">
                <Status options={state.cpuOptions as ApexOptions} series={state.cpuSeries}
                        data={Math.round(system.cpuUsage) + '%'}/>
            </div>
            <div className={styles.container__stats} id="ram">
                <Status options={state.ramOptions as ApexOptions} series={state.ramSeries}
                        data={`${bytesToGB(system.ramUsage).toFixed(2)}/${bytesToGB(system.ramTotal).toFixed(2)} GB`}/>
            </div>
        </div>
)
}