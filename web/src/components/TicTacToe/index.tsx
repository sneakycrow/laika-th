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
  // Sends a request to the backend to start a game and updates the state appropriately
  const startGame = () => {
    // TODO: Send request to API which is expected to give an ID of the game
    setGameID("some_id")
  }
  // A list of moves than have been made
  const makePlayerMove = (cell: number) => {
    // If a game is new, start a new game, then make the move
    if (!gameID) {
      startGame()
    }
    // Make sure the cell they're targeting is empty
    if (moves.find((move) => move.cell === cell)) {
      return
    }
    // Moves made client side should always be the player
    // Add the new move to the list of moves
    setMoves([...moves, { player: Player.User, cell }])
  }
  // Resets the game to it's beginning state
  const resetGame = () => {
    setGameID("")
    setMoves([])
  }

  return (
    <section className="grid grid-rows-game gap-20 items-center justify-center max-w-screen-md w-full">
      <GameBoard cells={CELLS_NUMBER} onCellClick={makePlayerMove} moves={moves} />
      <aside className="flex items-center justify-center w-full gap-4">
        {gameID ? (
          <Button onClick={resetGame}>Start Over</Button>
        ) : (
          <p className="px-2 py-4">Click any square to start a new game</p>
        )}
      </aside>
    </section>
  )
}

export default TicTacToe
