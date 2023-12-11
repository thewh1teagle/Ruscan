import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import SelectInterface, { Interface } from "./components/SelectInterface";
import ScanReport, { FoundHost } from "./components/ScanReport";
import ThemeToggle from "./components/ThemeToggle";
import { cx } from "./utils";

function App() {
  const [interfaces, setInterfaces] = useState<Interface[]>([])
  const [selected, setSelected] = useState<Interface>()
  const [report, setReport] = useState<FoundHost[]>()
  const [loading, setLoading] = useState(false)

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
    setLoading(true)
    const result = await invoke('scan', {interface: selected}) as FoundHost[]
    console.log('scan result => ', result)
    setReport(result)
    setLoading(false)
  }
  

  return (
    <div className="w-full h-[100vh] overflow-auto">
      <div className="absolute right-20 top-10">
        <ThemeToggle />
      </div>
      <div className="flex flex-col w-[300px] items-center m-auto mt-10">
        <SelectInterface interfaces={interfaces} onChange={i => setSelected(i)} />
        <button disabled={loading} onClick={() => scan()} className={cx("btn btn-primary w-full")}>{loading ? <span className="loading loading-spinner"/> : 'Scan'}</button>
      </div>
      <div className="max-w-[1000px] m-auto mt-10">
        {report && <ScanReport hosts={report} />}
      </div>
    </div>
  );
}

export default App;
