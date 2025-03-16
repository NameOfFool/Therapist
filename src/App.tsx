import styles from "./App.module.scss";
import Status from "./componenst/Status/Status.tsx";
import {useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import ApexCharts, {ApexOptions} from "apexcharts";
import ReactApexChart from "react-apexcharts";
interface SystemStatus {
    cpuUsage: number,
    ramUsage: number,
    ramTotal: number
}

function App() {

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
    const [state, setState] =  useState({

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



  return (
    <main className={styles.container}>
        <div>
            <h1>Overview</h1>
        </div>
        <div className={styles.container__stats} id="cpu">
            <ReactApexChart options={state.cpuOptions as ApexOptions} series={state.cpuSeries} type="line" height={350} />
        </div>
        <div className={styles.container__stats} id="ram">
            <ReactApexChart options={state.ramOptions as ApexOptions} series={state.ramSeries} type="line" height={350} />
        </div>
    </main>
  );
}

export default App;
