import { useState } from "react"
import { cx } from "../utils"

function limitString(s: string, maxLength: number = 25): string {
    if (s.length <= maxLength) {
        return s;
    } else {
        return s.substring(0, maxLength) + '..';
    }
}

export default function ScanField({ children }: { children: React.ReactNode }) {

    const [active, setActive] = useState(false)
    const [hover, setHover] = useState(false)

    function onClick() {
        setActive(true)
        navigator.clipboard.writeText(children?.toString() ?? '')
        setTimeout(() => setActive(false), 600)
    }


    return (
        <div className="relative" onMouseEnter={() => setHover(true)} onMouseLeave={() => setHover(false)}>
            <div className={cx("absolute -top-5 z-10 right-[50%] px-2 rounded-2xl translate-x-[50%]", active ? 'bg-green-500  text-base-100' : 'bg-slate-500 text-white', hover || active ? 'block' : 'hidden')}>
                {active ? (
                    <svg  xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" className="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                    </svg>
                ) : ''}
            </div>
            <code
                onClick={onClick}
                className="px-2 py-[0.2px] cursor-pointer rounded-1xl mt-1 text-xs bg-secondary-content">
                {limitString(children!.toString())}
            </code>
        </div>
    )
}