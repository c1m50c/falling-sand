import "./index.css";

type SideBarProps = {
    hidden: boolean;
}

function SideBar({ hidden }: SideBarProps) {
    return (
        <div className="SideBar">
            <h2>Sample Text</h2>
        </div>
    );
}

export default SideBar;