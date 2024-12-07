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
	Clear       State = 0b00000000
	Up                = 0b00000001
	Right             = 0b00000010
	Down              = 0b00000100
	Left              = 0b00001000
	Visited           = 0b00010000
	OutOfBounds       = 0b00100000
	Blocked           = 0b01000000
	CurrentPos        = 0b10000000
)

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
				app[x] = Up | Visited
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

	return board, Position{startingX, startingY - 1}
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

const print = false

func printBoard(board Board) {
	if !print {
		return
	}

	for _, line := range board {
		arr := make([]byte, len(line))
		for x, char := range line {
			if Visited&char == Visited {
				arr[x] = 'X'
			} else if CurrentPos&char == CurrentPos {
				arr[x] = '^'
			} else if Blocked&char == Blocked {
				arr[x] = '#'
			} else {
				arr[x] = '.'
			}
		}
		println(string(arr))
	}
	println("")
}
func checkVec(lines Board, pos Position, dir State) State {
	pos = move(pos, dir)
	if pos.y < 0 || pos.x < 0 || pos.y >= len(lines) || pos.x >= len(lines[0]) {
		return OutOfBounds
	}
	return lines[pos.y][pos.x]
}

var limit = 1000000

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
		fmt.Printf("done %d %d\n", boardState, state)
		return total
	}
	// current position has now been visited facing current direction
	board[pos.y][pos.x] |= state

	nextState := checkVec(board, pos, state)
	if nextState&OutOfBounds == OutOfBounds {
		return total
	}
	if nextState&Blocked == Blocked {
		if state >= Left {
			state = Up
		} else {
			// up -> right, right -> down, etc.
			state <<= 1
		}
	}

	oldState := board[pos.y][pos.x]
	board[pos.y][pos.x] = CurrentPos
	printBoard(board)
	board[pos.y][pos.x] = oldState

	pos = move(pos, state)
	return traverse(board, pos, state, total)
}

func main() {
	board, pos := getBoard()
	total := traverse(board, pos, Up, 1)
	fmt.Printf("total: %d\n\n", total)

	printBoard(board)
}
