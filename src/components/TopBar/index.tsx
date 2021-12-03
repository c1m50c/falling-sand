import { useState } from "react";
import SideBar from "../SideBar";
import "./index.css"
 
function TopBar() {
    const [sidebar_toggle, set_sidebar_toggle] = useState(false);

    return (
        <div className="TopBar">
            <div className="TopBarContainer">
                <svg viewBox="0 0 50 50" xmlns="http://www.w3.org/2000/svg" id="MenuSVG" onClick={() => set_sidebar_toggle(!sidebar_toggle)}>
                    <rect id="Bot" x="2" y="2" width="46" height="12" />
                    <rect id="Mid" x="2" y="19" width="46" height="12" />
                    <rect id="Top" x="2" y="36" width="46" height="12" />
                </svg>
                <h1><strong>Falling Sand</strong></h1>
            </div>
            {sidebar_toggle && (<SideBar hidden={sidebar_toggle} />)}
        </div>
    );
}

export default TopBar;