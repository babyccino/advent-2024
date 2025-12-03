package position

import (
	"fmt"
)

type Position struct {
	X int
	Y int
}
type Vector = Position

func (pos *Position) String() string {
	return fmt.Sprintf("Position{x: %d, y: %d}", pos.X, pos.Y)
}
func (pos *Position) Print() {
	fmt.Print(pos.String() + "\n")
}
func (pos *Position) Add(other Position) Position {
	return Position{pos.X + other.X, pos.Y + other.Y}
}
func (pos *Position) AddMult(other Position, mult int) Position {
	return Position{pos.X + mult*other.X, pos.Y + mult*other.Y}
}
func (pos *Position) Diff(other Position) Position {
	return Position{pos.X - other.X, pos.Y - other.Y}
}
func (pos *Position) AddInBounds(
	other Position,
	max Position,
) (Position, bool) {
	newX := pos.X + other.X
	newY := pos.Y + other.Y

	if newX < 0 || newX >= max.X || newY < 0 || newY >= max.Y {
		return Position{}, false
	}

	return Position{X: newX, Y: newY}, true
}

type Direction uint8

const (
	DownRight Direction = iota
	DownLeft
	UpLeft
	UpRight
	Up
	Down
	Left
	Right
)

func isDiagonal(direction Direction) bool {
	return direction <= UpRight
}
func isStraight(direction Direction) bool {
	return direction >= Up
}

var (
	DownRightVec = Vector{1, 1}
	DownLeftVec  = Vector{-1, 1}
	UpLeftVec    = Vector{-1, -1}
	UpRightVec   = Vector{1, -1}
	UpVec        = Vector{0, -1}
	DownVec      = Vector{0, 1}
	LeftVec      = Vector{-1, 0}
	RightVec     = Vector{1, 0}
)

var directionArray = [...]Vector{
	DownRightVec, DownLeftVec,
	UpLeftVec, UpRightVec,
	UpVec, DownVec,
	LeftVec, RightVec,
}
var StraightVecs = directionArray[Up:]

func DirectionToVec(dir Direction) Vector {
	return directionArray[dir]
}
