import ReactDOM from "react-dom/client";
import App from "./App";
import {BrowserRouter, Routes, Route} from "react-router";
import Overview from "./views/Overview/Overview.tsx";
import Ports from "./views/Ports/Ports.tsx";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <BrowserRouter>
      <Routes>
          <Route path="/" element={<App />}>
              <Route index element={<Overview/>}/>
              <Route path="ports" element={<Ports/>}/>
          </Route>
      </Routes>
  </BrowserRouter>
);
