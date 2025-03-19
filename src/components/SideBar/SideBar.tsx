import styles from './SideBar.module.scss'
import {NavLink} from "react-router";

export default function SideBar(){
    return (
        <nav className={styles.container}>
            <NavLink className={styles.container__link} to="/">Overview</NavLink>
            <NavLink className={styles.container__link}  to="/port">Port Setting</NavLink>
        </nav>
    )
}