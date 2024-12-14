// import { useState } from "react";

function MainToggle(props) {

    function handleChange(event) {
        console.log(event.target.checked);
        let newAppData = Object.create(props.appData);
        Object.assign(newAppData, {onOff: event.target.checked});
        props.setAppData(newAppData);
    }

    return (
        <>
            <label className="relative flex justify-center items-center p-2 text-xl py-4 font-bold">
                <h2 className="text-right px-4">Off</h2>
                <input type="checkbox" className="absolute left-1/2 -translate-x-1/2 w-full h-full peer appearance-none rounded-md" onChange={handleChange} checked={props.appData.onOff} />
                <span className="w-32 h-10 flex grow-0 items-center ml-4 p-1 bg-slate-800 rounded-lg duration-300 ease-in-out peer-checked:bg-cyan-700 after:w-20 after:h-8 after:bg-slate-400 after:rounded-lg after:shadow-md after:duration-300 peer-checked:after:bg-slate-100 peer-checked:after:translate-x-10"></span>
                <h2 className="text-left px-4">On</h2>
            </label>
        </>
    );
}

export default MainToggle;