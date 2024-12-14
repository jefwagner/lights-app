import { useState } from "react";

function Slider(props) {

    let [value, setValue] = useState(0.5*props.min + 0.5*props.max);

    function handleChange(event) {
        setValue(event.target.value);
    }

    return (
        <>
        <div className="p-2 text-xl py-1 font-bold">
            <label htmlFor={"slider-" + props.label} className="block mb-2 text-left text-gray-900 dark:text-white"> {props.label}: {value} </label>
            <input 
                id={"slider-" + props.label} 
                type="range" 
                min={props.min}
                max={props.max} 
                value={value}
                onChange={handleChange}
                className="range accent-cyan-700 w-full h-2 bg-slate-200 rounded-lg appearance-none cursor-pointer dark:bg-slate-700"
            />
        </div>
        </>
    );
}

export default Slider;