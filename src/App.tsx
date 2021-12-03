import { MutableRefObject, useRef } from "react";
import TopBar from "./components/TopBar";
import "./App.css";

function App() {
    const canvas: MutableRefObject<null | HTMLCanvasElement> = useRef(null);
    
    return (
        <div className="App">
            <TopBar />
            <canvas ref={canvas} />
        </div>
    );
}

export default App;