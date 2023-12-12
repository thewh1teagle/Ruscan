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
    <ul role="list" className="divide-y divide-neutral">
      {hosts.map(host => (
        <li key={host.host} className="flex justify-between gap-x-6 py-5 flex-col text-center lg:flex-row items-center">
          <div className="w-[500px] lg:w-[100px] flex items-center justify-center">
            <div className="rounded-full w-3 h-3 bg-success" />
          </div>
          <div className="min-w-0 flex-auto w-[500px] lg:w-[100px] overflow-auto">
            <p className="text-sm font-semibold leading-6 ">IP</p>
            <ScanField>{host.host}</ScanField>
          </div>
          <div className="min-w-0 flex-auto w-[500px] lg:w-[100px] overflow-auto">
            <p className="text-sm font-semibold leading-6">HOSTNAME</p>
            <ScanField>{host?.hostname == '' ? '-' : host.hostname}</ScanField>
          </div>
          <div className="min-w-0 flex-auto w-[500px] lg:w-[100px] overflow-auto">
            <p className="text-sm font-semibold leading-6 ">MAC</p>
            {/* <div className="uppercase"><ScanField>aa:bb:cc:dd:ff</ScanField></div> */}
            <div className="uppercase"><ScanField>{host.mac}</ScanField></div>
          </div>
          <div className="min-w-0 flex-auto w-[500px] lg:w-[100px] overflow-auto">
            <p className="text-sm font-semibold leading-6 ">Vendor</p>
            <ScanField>{host.vendor == '' ? '-' : host.vendor}</ScanField>
          </div>
        </li>
      ))}

    </ul>
  )
}