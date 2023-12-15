import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import SelectInterface, { Interface } from "./components/SelectInterface";
import ScanReport, { FoundHost } from "./components/ScanReport";
import ThemeToggle from "./components/ThemeToggle";
import { cx } from "./utils";
import Progress from "./components/Progress";

function App() {
  const [interfaces, setInterfaces] = useState<Interface[]>([])
  const [selected, setSelected] = useState<Interface>()
  const [report, setReport] = useState<FoundHost[]>()
  const [loading, setLoading] = useState(false)

  async function loadInterfaces() {
    const found = await invoke('get_interfaces') as Interface[]
    setInterfaces(found)
    setSelected(found[0])
  }

  useEffect(() => {
    loadInterfaces()
  }, [])

  async function scan() {
    setReport(undefined)
    setLoading(true);
  
    try {
      const result = await invoke('scan', { interface: selected }) as FoundHost[];
      setReport(result);
    } catch (error) {
      console.error('Error during scan:', error);
    } finally {
      setLoading(false);
    }
  }
  
  return (
    <div className="w-full h-[100vh] overflow-auto no-scrollbar">
      <div className="absolute lg:right-20 lg:top-10 top-8 right-8">
        <ThemeToggle />
      </div>
      <div className="flex flex-col w-[300px] items-center m-auto mt-10">
        <SelectInterface interfaces={interfaces} onChange={i => setSelected(i)} />
        <button disabled={loading} onClick={() => scan()} className={cx("btn btn-primary w-full mt-1.5")}>{loading ? <span className="loading loading-spinner" /> : 'Scan'}</button>
        {loading && <Progress />}
      </div>
      <div className="max-w-[1500px] m-auto mt-10">
        {report && <ScanReport hosts={report} />}
      </div>

    </div>
  );
}

export default App;
