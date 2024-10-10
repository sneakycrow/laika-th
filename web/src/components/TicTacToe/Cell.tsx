import { Player, renderPlayerToString } from "."

type CellProps = {
    player?: Player
    onClick: () => void
}

const Cell = (props: CellProps) => {
    return (
        <div
            onClick={props.onClick}
            className="w-16 h-16 dark:bg-white dark:text-black text-2xl text-center hover:cursor-pointer hover:scale-105 transition-transform"
        >
            {renderPlayerToString(props.player)}
        </div>
    )
}

export default Cell
