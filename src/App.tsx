import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import SelectInterface, { Interface } from "./components/SelectInterface";
import ScanReport, { FoundHost } from "./components/ScanReport";
import ThemeToggle from "./components/ThemeToggle";

function App() {
  const [interfaces, setInterfaces] = useState<Interface[]>([])
  const [selected, setSelected] = useState<Interface>()
  const [report, setReport] = useState<FoundHost[]>()

  async function loadInterfaces() {
    const found = await invoke('get_interfaces') as Interface[]
    console.log(interfaces)
    setInterfaces(found)
    setSelected(found[0])
  }

  useEffect(() => {
    loadInterfaces()
  }, [])

  async function scan() {
    const result = await invoke('scan', {interface: selected}) as FoundHost[]
    console.log('scan result => ', result)
    setReport(result)
  }
  

  return (
    <div className="w-full h-[100vh] overflow-auto">
      <div className="absolute right-20 top-10">
        <ThemeToggle />
      </div>
      <div className="flex flex-col w-[300px] items-center m-auto mt-10">
        <SelectInterface interfaces={interfaces} onChange={i => setSelected(i)} />
        <button onClick={() => scan()} className="btn btn-primary w-full">Scan</button>
      </div>
      <div className="max-w-[1000px] m-auto mt-10">
        {report && <ScanReport hosts={report} />}
      </div>
    </div>
  );
}

export default App;
