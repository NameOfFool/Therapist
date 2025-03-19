import styles from "./App.module.scss";
import {Outlet} from "react-router";
import SideBar from "./components/SideBar/SideBar.tsx";


function App() {

  return (
    <div className={styles.container}>
        <SideBar/>
        <main className={styles.container__main}>
            <Outlet/>
        </main>
    </div>
  );
}

export default App;
