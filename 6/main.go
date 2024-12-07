package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type State uint
type Position struct {
	x int
	y int
}
type Row = []State
type Board = []Row

const (
	Clear        State = 0b000000000
	Up                 = 0b000000001
	Right              = 0b000000010
	Down               = 0b000000100
	Left               = 0b000001000
	Visited            = 0b000010000
	OutOfBounds        = 0b000100000
	Blocked            = 0b001000000
	CurrentPos         = 0b010000000
	AddedBlocked       = 0b101000000
)

func turn(state State) State {
	if state >= Left {
		return Up
	} else {
		// up -> right, right -> down, etc.
		return state << 1
	}
}

func getBoard() (Board, Position) {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	board := make(Board, 0)
	y := 0
	var startingX, startingY int
	for scanner.Scan() {
		text := scanner.Text()
		app := make(Row, len(text))
		for x, char := range text {
			if char == '#' {
				app[x] = Blocked
			} else if char == '^' {
				startingX = x
				startingY = y
			}
		}
		board = append(board, app)
		y += 1
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return board, Position{startingX, startingY}
}

func copyBoard(board Board) Board {
	duplicate := make(Board, len(board))
	for i := range board {
		duplicate[i] = make(Row, len(board[i]))
		copy(duplicate[i], board[i])
	}
	return duplicate
}

func directionToVec(dir State) (int, int) {
	if dir == Up {
		return -1, 0
	}
	if dir == Down {
		return 1, 0
	}
	if dir == Left {
		return 0, -1
	}
	if dir == Right {
		return 0, 1
	}
	panic("ahhhh")
}

func move(pos Position, dir State) Position {
	moveY, moveX := directionToVec(dir)
	pos.x += moveX
	pos.y += moveY
	return pos
}

func printBoard(board Board, pos Position) {
	_printBoard(board, pos)
}
func debugPrintBoard(board Board, pos Position) {
	// _printBoard(board)
}
func _printBoard(board Board, pos Position) {
	oldState := board[pos.y][pos.x]
	board[pos.y][pos.x] = CurrentPos

	for _, line := range board {
		arr := make([]byte, len(line))
		for x, char := range line {
			if Visited&char == Visited {
				arr[x] = 'X'
			} else if CurrentPos&char == CurrentPos {
				arr[x] = '^'
			} else if AddedBlocked&char == AddedBlocked {
				arr[x] = '0'
			} else if Blocked&char == Blocked {
				arr[x] = '#'
			} else {
				arr[x] = '.'
			}
		}
		println(string(arr))
	}
	println("")

	board[pos.y][pos.x] = oldState
}

func checkVec(lines Board, pos Position, dir State) State {
	pos = move(pos, dir)
	if pos.y < 0 || pos.x < 0 || pos.y >= len(lines) || pos.x >= len(lines[0]) {
		return OutOfBounds
	}
	return lines[pos.y][pos.x]
}

var limit = 1000000

// p1

func traverse(board Board, pos Position, state State, total int) int {
	limit -= 1
	if limit == 0 {
		return total
	}
	boardState := board[pos.y][pos.x]
	if (boardState & Visited) == Clear {
		total += 1
		board[pos.y][pos.x] |= Visited
	}
	// current position has been visited facing current direction
	if boardState&state == state {
		return total
	}
	// current position has now been visited facing current direction
	board[pos.y][pos.x] |= state

	nextState := checkVec(board, pos, state)
	if nextState&OutOfBounds == OutOfBounds {
		return total
	}
	if nextState&Blocked == Blocked {
		state = turn(state)
	}

	debugPrintBoard(board, pos)

	return traverse(board, move(pos, state), state, total)
}

func p1() {
	board, initialPos := getBoard()
	total := traverse(board, initialPos, Up, 0)
	printBoard(board, initialPos)
	fmt.Printf("total positions visited: %d", total)
}

// p2

func getVisited(board Board, initialPosition Position) []Position {
	ret := make([]Position, 0)
	for y, line := range board {
		for x, char := range line {
			if initialPosition.x == y && initialPosition.y == x {
				continue
			}
			if Visited&char == Visited {
				ret = append(ret, Position{x, y})
			}
		}
	}
	return ret
}

func detectCycle(board Board, pos Position, state State) bool {
	boardState := board[pos.y][pos.x]
	if boardState&Visited == Clear {
		board[pos.y][pos.x] |= Visited
	}
	// current position has been visited facing current direction
	if boardState&state == state {
		return true
	}
	// current position has now been visited facing current direction
	board[pos.y][pos.x] |= state

	// loop to handle corners (didn't matter in p1 apparently)
	for {
		nextState := checkVec(board, pos, state)
		if nextState&OutOfBounds == OutOfBounds {
			return false
		}
		if nextState&Blocked != Blocked {
			break
		}

		state = turn(state)
	}

	debugPrintBoard(board, pos)

	return detectCycle(board, move(pos, state), state)
}

func testCycle() {
	board, initialPos := getBoard()
	cycle := detectCycle(board, initialPos, Up)
	printBoard(board, initialPos)
	fmt.Printf("[test] had cycle: %t", cycle)
}

func p2() {
	initialBoard, initialPos := getBoard()

	var visited []Position
	{
		board := copyBoard(initialBoard)
		total := traverse(board, initialPos, Up, 0)

		printBoard(board, initialPos)
		fmt.Printf("total: %d\n\n", total)
		visited = getVisited(board, initialPos)
	}

	total := 0
	shouldPrintBoard := false

	for _, pos := range visited {
		newBoard := copyBoard(initialBoard)
		newBoard[pos.y][pos.x] = AddedBlocked

		var boardPrint Board
		if shouldPrintBoard {
			boardPrint = copyBoard(newBoard)
			boardPrint[initialPos.y][initialPos.x] = CurrentPos
		}

		if detectCycle(newBoard, initialPos, Up) {
			total += 1
			if shouldPrintBoard && boardPrint != nil {
				printBoard(boardPrint, initialPos)
				printBoard(newBoard, initialPos)
				println("\n")
			}
		}
	}

	fmt.Printf("total possible cycles: %d\n", total)
}

func main() {
	p2()
}
