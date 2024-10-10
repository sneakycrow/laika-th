import { useState } from "react"
import Button from "../Button"
import GameBoard from "./GameBoard"

// Each game of TicTacToe should have 9 cells
const CELLS_NUMBER = 9

export enum Player {
    Computer,
    User,
}

export const renderPlayerToString = (player: Player | undefined): string => {
    switch (player) {
        case Player.Computer: {
            return "o"
        }
        case Player.User: {
            return "x"
        }
        case undefined:
        default: {
            return ""
        }
    }
}

export type Move = {
    player: Player
    cell: number // Should be a number between 1 and 9
}

const TicTacToe = () => {
    // The ID of the current game being played
    const [gameID, setGameID] = useState("")
    const [moves, setMoves] = useState<Move[]>([])
    const [isDetailsShown, setShowDetails] = useState(false)
    const [isPlayersTurn, setIsPlayersTurn] = useState(true)
    // Sends a request to the backend to start a game and updates the state appropriately
    const startGame = async () => {
        const res = await fetch(`${import.meta.env.VITE_API_URL}/game`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                player_id: "some_id",
            }),
        })
        if (!res.ok) {
            throw "Bad response from API"
        }
        // The response is expected to contain a game ID we can reference in future updates
        const json = await res.json()
        if (!json.id) {
            // Somethings gone wrong, throw an error for now
            throw "No game ID in response"
        }
        setGameID(json.id)
    }
    // A list of moves than have been made
    const makePlayerMove = async (cell: number) => {
        // If a game is new, start a new game, then make the move
        if (!gameID) {
            await startGame()
        }
        // If it's not the player's turn, they cannot make a move
        if (!isPlayersTurn) {
            return
        }
        // Make sure the cell they're targeting is empty
        if (moves.find((move) => move.cell === cell)) {
            return
        }
        // Finished validating, start making move
        // Update players turn first so we can allow the computer to make a move
        setIsPlayersTurn(false)
        // Moves made client side should always be the player
        // Add the new move to the list of moves
        setMoves([...moves, { player: Player.User, cell }])
        // TODO: Send our move to the computer so they can respond
        // TODO: Push the computers move to our moves list
        // Allow the player to make their next move
        // Temporary wait to test loading UI
        await new Promise((resolve) => setTimeout(resolve, 3000))
        setIsPlayersTurn(true)
    }
    // Resets the game to it's beginning state
    const resetGame = () => {
        setMoves([])
        startGame()
    }
    // Toggles the game stats being displayed
    const toggleStats = () => {
        setShowDetails(!isDetailsShown)
    }

    return (
        <section className="grid grid-cols-2 gap-20 items-center justify-center max-w-screen-lg w-full">
            <div
                className={`transition-opacity ${!isPlayersTurn ? "opacity-50" : ""}`}
            >
                <GameBoard
                    cells={CELLS_NUMBER}
                    onCellClick={makePlayerMove}
                    moves={moves}
                    isLoading={!isPlayersTurn}
                />
            </div>
            <aside className="h-full flex flex-col items-start justify-start min-w-max gap-4 col-start-2">
                <div>
                    <p
                        className={`text-2xl ${isPlayersTurn ? "underline" : ""}`}
                    >
                        You -{" "}
                        <span className="font-semibold">
                            {renderPlayerToString(Player.User)}
                        </span>
                    </p>
                    <p
                        className={`text-2xl ${!isPlayersTurn ? "underline animate-bounce" : ""}`}
                    >
                        Computer -{" "}
                        <span className="font-semibold">
                            {renderPlayerToString(Player.Computer)}
                        </span>
                    </p>
                </div>
                {gameID ? (
                    <>
                        <Button onClick={resetGame}>Start Over</Button>
                        <button className="underline" onClick={toggleStats}>
                            {isDetailsShown ? "Hide" : "Show"} Details
                        </button>
                    </>
                ) : (
                    <p>Select any square to start</p>
                )}
                {gameID && isDetailsShown && (
                    <p className="text-xs">
                        Game ID:{" "}
                        <span className="underline text-base">{gameID}</span>
                    </p>
                )}
            </aside>
        </section>
    )
}

export default TicTacToe
