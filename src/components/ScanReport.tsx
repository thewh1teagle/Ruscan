import ScanField from "./ScanField"

export interface FoundHost {
  host: string,
  mac: string,
  vendor: string,
  hostname: string,
}
interface ScanReportProps {
  hosts: FoundHost[]
}
export default function ScanReport({ hosts }: ScanReportProps) {
  return (
    <ul role="list" className="divide-y divide-neutral mx-20">
      {hosts.map(host => (
        <li key={host.host} className="flex justify-between gap-x-6 py-5 flex-col text-center md:flex-row items-center">
          <div className="w-[500px] lg:w-[100px] flex items-center justify-center">
            <div className="rounded-full w-3 h-3 bg-success" />
          </div>
          <div className="min-w-0 flex-auto w-[500px] lg:w-[100px] overflow-auto scrollbar">
            <p className="text-sm font-semibold leading-6 text-neutral-content">IP</p>
            <ScanField>{host.host}</ScanField>
          </div>
          <div className="min-w-0 flex-auto w-[500px] lg:w-[100px] overflow-auto scrollbar">
            <p className="text-sm font-semibold leading-6 text-neutral-content">HOSTNAME</p>
            <ScanField>{host?.hostname == '' ? '-' : host.hostname}</ScanField>
          </div>
          <div className="min-w-0 flex-auto w-[500px] lg:w-[100px] overflow-auto scrollbar">
            <p className="text-sm font-semibold leading-6 text-neutral-content">MAC</p>
            {/* <div className="uppercase"><ScanField>aa:bb:cc:dd:ff</ScanField></div> */}
            <div className="uppercase"><ScanField>{host.mac}</ScanField></div>
          </div>
          <div className="min-w-0 flex-auto w-[500px] lg:w-[100px] overflow-auto scrollbar">
            <p className="text-sm font-semibold leading-6 text-neutral-content">Vendor</p>
            <ScanField>{host.vendor == '' ? '-' : host.vendor}</ScanField>
          </div>
        </li>
      ))}

    </ul>
  )
}