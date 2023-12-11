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
  const [progress, setProgress] = useState(0)



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
    setReport(undefined)
    setLoading(true);
    setProgress(0);
  
    const intervalDuration = 20;
    const totalProgressSteps = 1;
    const totalDuration = 80; // almost 10 seconds
    const progressStep = totalProgressSteps / (totalDuration / intervalDuration);
  
    const progressInterval = setInterval(() => {
      setProgress((prevProgress) => {
        const newProgress = prevProgress + progressStep;
  
        // Check if the progress has reached 100
        if (newProgress >= 100) {
          clearInterval(progressInterval); // Stop the interval
          return 100;
        }
  
        return newProgress;
      });
    }, intervalDuration);
  
    try {
      const result = await invoke('scan', { interface: selected }) as FoundHost[];
      console.log('scan result => ', result);
      setReport(result);
    } catch (error) {
      console.error('Error during scan:', error);
    } finally {
      clearInterval(progressInterval);
      setProgress(100); // Ensure progress is set to 100 after completion
      setLoading(false);
    }
  }
  


  return (
    <div className="w-full h-[100vh] overflow-auto">
      <div className="absolute right-20 top-10">
        <ThemeToggle />
      </div>
      <div className="flex flex-col w-[300px] items-center m-auto mt-10">
        <SelectInterface interfaces={interfaces} onChange={i => setSelected(i)} />
        <button disabled={loading} onClick={() => scan()} className={cx("btn btn-primary w-full mt-1.5")}>{loading ? <span className="loading loading-spinner" /> : 'Scan'}</button>
        {progress > 0 && loading && (
          <div className="flex justify-center mt-3 w-full">
            <progress className="progress progress-primary w-[100%]" value={progress} max="100"></progress>
          </div>
        )}
      </div>


      <div className="max-w-[1000px] m-auto mt-10">
        {report && <ScanReport hosts={report} />}
      </div>

    </div>
  );
}

export default App;
