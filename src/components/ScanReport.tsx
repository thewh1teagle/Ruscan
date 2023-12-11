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
export default function ScanReport({hosts}: ScanReportProps) {
    return (
        <ul role="list" className="divide-y divide-gray-100">
        {hosts.map(host => (
          <li key={host.host} className="flex justify-between gap-x-6 py-5 flex-col text-center lg:flex-row items-center">
           <svg xmlns="http://www.w3.org/2000/svg" className='h-8 w-8 fill-primary' viewBox="0 0 640 512"><path d="M384 96V320H64L64 96H384zM64 32C28.7 32 0 60.7 0 96V320c0 35.3 28.7 64 64 64H181.3l-10.7 32H96c-17.7 0-32 14.3-32 32s14.3 32 32 32H352c17.7 0 32-14.3 32-32s-14.3-32-32-32H277.3l-10.7-32H384c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64H64zm464 0c-26.5 0-48 21.5-48 48V432c0 26.5 21.5 48 48 48h64c26.5 0 48-21.5 48-48V80c0-26.5-21.5-48-48-48H528zm16 64h32c8.8 0 16 7.2 16 16s-7.2 16-16 16H544c-8.8 0-16-7.2-16-16s7.2-16 16-16zm-16 80c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16s-7.2 16-16 16H544c-8.8 0-16-7.2-16-16zm32 160a32 32 0 1 1 0 64 32 32 0 1 1 0-64z"/></svg>
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
              <div className="uppercase"><ScanField>aa:bb:cc:dd:ff</ScanField></div>
              {/* <div className="uppercase"><ScanField>{host.mac}</ScanField></div> */}
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