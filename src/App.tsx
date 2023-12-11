import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import SelectInterface, { Interface } from "./components/SelectInterface";
import ScanReport, { FoundHost } from "./components/ScanReport";

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
    <div className="w-full">
      <div className="flex flex-col w-[300px] items-center m-auto">
        <SelectInterface interfaces={interfaces} onChange={i => setSelected(i)} />
        <button onClick={() => scan()} className="btn w-full">Scan</button>
      </div>
      {report && <ScanReport hosts={report} />}
    </div>
  );
}

export default App;
