import { useState } from 'react'
import './App.css'
import MainToggle from './components/MainToggle'
import ModeSelect from './components/ModeSelect'
import Slider from './components/Slider'

function App() {
  const [appData, setAppData] = useState({
    onOff: false,
    modes: ["foo", "bar", "baz", "boo"],
    selected: "bar",
    parameters: {},
  });
  const [onOff, setOnOff] = useState(false);

  let connData = {
    mode: import.meta.env.MODE,
    ws_addr: undefined,
  }
  if (connData.mode == "production") {
    connData.ws_addr = "/ws"
  } else {
    connData.ws_addr = "https://localhost:8000/ws"
  }

  return (
    <>
      <h1 className="text-3xl font-bold bg-cyan-700 rounded-lg">
          Christmas Lights
      </h1>
      <MainToggle left="Off" right="On" appData={appData} setAppData={setAppData} />
      <ModeSelect appData={appData} setAppData={setAppData} />
      <h2 className="text-xl font-bold">State</h2>
      <p>{ appData.onOff? "on": "off" }</p>
      <p> mode: { appData.selected? appData.selected : "none" }</p>
      <Slider label="Foo" min="25" max="175" />
    </>
  )
}

export default App
