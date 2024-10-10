import type { Move, Player } from "."
import Cell from "./Cell"

type GameBoardProps = {
    cells: number
    onCellClick: (cell: number) => void
    moves: Move[]
    isLoading?: boolean
}

const GameBoard = (props: GameBoardProps) => {
    // A function for determining which player (if any) has played a specific cell by its "id"
    // The "ID" is just the position of the cell, 1 - 9
    const getPlayer = (cell: number): Player | undefined => {
        return props.moves.find((move) => move.cell === cell)?.player
    }

    return (
        <div className="grid grid-cols-3 gap-2 w-64 h-64 items-center justify-center mx-auto">
            {[...Array(props.cells)].map((_, index) => (
                <Cell
                    key={index}
                    onClick={() => props.onCellClick(index + 1)}
                    player={getPlayer(index + 1)}
                />
            ))}
        </div>
    )
}

export default GameBoard
