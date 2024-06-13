export interface Interface {
    name: string
    friendly_name: string
    index: number
}

interface SelectInterfaceProps {
    interfaces: Interface[]
    onChange: (selected: Interface) => void
}

export default function SelectInterface({interfaces, onChange}: SelectInterfaceProps) {

    function localOnChange(event: any) {
        const indexStr = event.target.value as string
        const index = Number(indexStr)
        const selected = interfaces[index]
        onChange(selected)
    }
    return (
        <label className="form-control w-full max-w-xs">
            <div className="label">
                <span className="label-text">Select Network Interface</span>
            </div>
            <select onChange={e => localOnChange(e)} className="select select-bordered">
                {interfaces.map((i, index) => (
                    <option key={i.index} value={index}>{i.friendly_name || i.name}</option>
                ))}
            </select>
        </label>
    )
}