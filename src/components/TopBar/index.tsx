import "./index.css"
 
function TopBar() {
    return (
        <div className="TopBar">
            <svg viewBox="0 0 50 50" fill="none" xmlns="http://www.w3.org/2000/svg" id="MenuSVG">
                <rect id="Bot" x="2" y="2" width="46" height="12" />
                <rect id="Mid" x="2" y="19" width="46" height="12" />
                <rect id="Top" x="2" y="36" width="46" height="12" />
            </svg>
            <h1><strong>Falling Sand</strong></h1>
        </div>
    );
}

export default TopBar;