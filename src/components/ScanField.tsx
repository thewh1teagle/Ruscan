import { useState } from "react"
import { cx } from "../utils"

export default function ScanField({children}: {children: React.ReactNode}) {

    const [active, setActive] = useState(false)

    function onClick() {
        setActive(true)
        navigator.clipboard.writeText(children?.toString() ?? '')
        setTimeout(() => setActive(false), 600)
    }

    return (
        <code 
            onClick={onClick} 
            className={cx("px-1 py-[0.2px] cursor-pointer rounded-2xl", active ? "bg-green-300 text-black" : "hover:bg-slate-500 hover:text-white")}>
            {children}
        </code>
    )
}