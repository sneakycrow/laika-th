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
            return "x"
        }
        case Player.User: {
            return "o"
        }
        case undefined:
        default: {
            return ""
        }
    }
}

// A move in a game
export type Move = {
    // The player that made the move
    player: Player
    // The cell the move was made in
    cell: number // Should be a number between 1 and 9
    // The turn the move was made on
    turn: number
}

// The expected response from both the start and update endpoints
type ServerResponse = {
    id: string // The game ID
    moves: ServerMove[] // The move history of the game
    players: ServerPlayer[] // Players participating in the game
    status: ServerGameStatus // Status of the game
    winner: string | null
}

type ServerPlayer = { Player: string } | string // A player will have an ID associated with them
type ServerMove = {
    position: number // The cell the move was made in
    turn: number // The turn the move was made on
    player: { Player: string } | string // The player that made the move
}
type ServerGameStatus = "NotStarted" | "InProgress" | "Complete"

const TicTacToe = () => {
    // The ID of the current game being played
    const [gameID, setGameID] = useState("")
    // TODO: Make this editable by the user
    const playerID = "human"
    const [gameStatus, setGameStatus] = useState<ServerGameStatus>("NotStarted")
    const [gameWinner, setGameWinner] = useState<string | null>(null)
    const [moves, setMoves] = useState<Move[]>([])
    const [isDetailsShown, setShowDetails] = useState(false)
    const [isPlayersTurn, setIsPlayersTurn] = useState(true)
    // Sends a request to the backend to start a game and updates the state appropriately
    const startGame = async (first_move?: Move) => {
        let reqBody: {
            player_id: string
            move_position?: number
            turn?: number
        } = {
            player_id: playerID,
        }
        // If a first move exists, push additional move data
        if (first_move) {
            reqBody.move_position = first_move.cell
            reqBody.turn = 0
        }
        const res = await fetch(`${import.meta.env.VITE_API_URL}/game`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(reqBody),
        })
        if (!res.ok) {
            throw "Bad response from API"
        }
        // The response is expected to contain a game ID we can reference in future updates
        const json = (await res.json()) as ServerResponse
        // Runtime validation
        if (!json.id) {
            // Somethings gone wrong, throw an error for now
            throw "No game ID in response"
        }
        setGameID(json.id)
        // The response should contain a full validated history of moves
        // Runtime validation
        if (!json.moves) {
            throw "No move history in response"
        }
        // Sync the moves from the server
        const updatedMoves = parseServerMoves(json.moves)
        setMoves(updatedMoves)
        // We expect the game to be in progress, but this can be extended later for two human players
        // For example, starting a game but letting your opponent make the first move
        // Runtime validation
        if (!json.status) {
            throw "No game status in response"
        }
        setGameStatus(json.status)
    }
    // Posts a new move to an existing game
    const updateGame = async (move: Move, gameID: String) => {
        const reqBody = {
            move_position: move.cell,
            player_id: playerID,
        }
        // TODO: These fetches to the API can probably be made into a wrapper
        const res = await fetch(
            `${import.meta.env.VITE_API_URL}/game/${gameID}`,
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(reqBody),
            },
        )
        const json = (await res.json()) as ServerResponse
        // Runtime validation
        if (!json.moves) {
            throw "No move history in response"
        }
        // Sync the moves from the server
        const updatedMoves = parseServerMoves(json.moves)
        setMoves(updatedMoves)
        // Additionally, we want to sync the status of the game in case someone won
        // Runtime validation
        if (!json.status) {
            throw "No game status in response"
        }
        setGameStatus(json.status)
        // Additionally, check if there's a winner
        // Runtime validation
        if (!json.winner) {
            throw "No game status in response"
        }
        setGameWinner(json.winner)
    }
    // Parses moves from the server into client side moves
    const parseServerMoves = (moves: ServerMove[]): Move[] => {
        /// Pulls and parses a player out of a server move
        const getPlayerFromServerMove = (move: ServerMove): Player => {
            // Check for a custom player id
            if (typeof move.player === "object") {
                // This is presumed to be a player with a custom ID
                // We only need to represent that they're a player right now, TBD using custom IDs
                return Player.User
            }
            // If it's the computer, it should just be a string 'Computer'
            if (move.player === "Computer") {
                return Player.Computer
            }
            // If we made it here, we've encountered something unexpected
            throw "Unexpected error parsing player from server move"
        }
        return moves.map((move: ServerMove) => ({
            cell: move.position,
            player: getPlayerFromServerMove(move),
            turn: move.turn,
        })) as Move[]
    }
    // Sends a new move from the player (client) to the server
    const makePlayerMove = async (cell: number) => {
        // If the game is completed, no moves can be made
        if (gameStatus === "Complete") {
            return
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
        let newMove = { player: Player.User, cell, turn: 1 }
        setMoves([...moves, newMove])
        // If this is a new game, we can start the game initialized with our move
        // If a game is new, start a new game, then make the move
        // Otherwise, update the existing game
        try {
            if (!gameID) {
                await startGame(newMove)
            } else {
                await updateGame(newMove, gameID)
            }
        } catch (e) {
            console.error("Server error starting/updating game: ", e)
        }
        // Allow the player to make their next move
        setIsPlayersTurn(true)
    }
    // Resets the game to it's beginning state
    const resetGame = () => {
        setMoves([])
        startGame()
        setGameStatus("InProgress")
        setGameWinner(null)
    }
    // Toggles the game stats being displayed
    const toggleStats = () => {
        setShowDetails(!isDetailsShown)
    }

    return (
        <section className="grid grid-cols-2 gap-20 max-w-screen-lg w-full">
            <div className="col-span-2">
                {gameStatus === "Complete" &&
                    (gameWinner !== null ? (
                        <p className="text-huge font-extrabold text-blue-500 w-full text-center">
                            {gameWinner === "Computer"
                                ? renderPlayerToString(Player.Computer)
                                : renderPlayerToString(Player.User)}
                            {"'s "}
                            have won!
                        </p>
                    ) : (
                        <p className="text-huge font-extrabold text-red-500 w-full text-center">
                            DRAW!
                        </p>
                    ))}
            </div>
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
                        className={`text-2xl ${isPlayersTurn ? "underline" : ""} ${gameWinner && gameWinner !== "Computer" ? "text-blue-500" : ""}`}
                    >
                        You -{" "}
                        <span className="font-semibold">
                            {renderPlayerToString(Player.User)}
                        </span>
                    </p>
                    <p
                        className={`text-2xl ${!isPlayersTurn ? "underline animate-bounce" : ""} ${gameWinner && gameWinner === "Computer" ? "text-blue-500" : ""}`}
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
