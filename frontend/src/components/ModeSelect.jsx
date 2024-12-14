function ModeSelect (props) {

    function handleChange(event) {
        console.log(event.target.value);
        let newAppData = Object.create(props.appData);
        Object.assign(newAppData, {selected: event.target.value});
        props.setAppData(newAppData);
    }

    return (
        <>
        <div className="flex">
            <label htmlFor="mode" className="block mb-2 text-lg font-bold px-2 self-center">Mode</label>
            <select 
                id="mode"
                value={ props.appData.selected? props.appData.selected: ""}
                onChange={handleChange}
                className="bg-slate-700 border border-slate-600 text-lg rounded-lg flex-grow focus:ring-cyan-700 focus:border-cyan-700 block p-2.5"
            >
                {props.appData.modes.map((mode, index) => (
                <option key={index} value={mode}>
                    {mode}
                </option>
                ))}                
            </select>
        </div>
        </>
    );
}

export default ModeSelect;