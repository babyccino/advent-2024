package position

import "fmt"

type Position struct {
	X int
	Y int
}

func (pos *Position) Print() {
	fmt.Printf("Position{x: %d, y: %d}\n", pos.X, pos.Y)
}
func (pos *Position) Add(other Position) Position {
	return Position{pos.X + other.X, pos.Y + other.Y}
}
func (pos *Position) AddMult(other Position, mult int) Position {
	return Position{pos.X + mult*other.X, pos.Y + mult*other.Y}
}
func (pos *Position) Diff(other Position) Position {
	return pos.AddMult(other, -1)
}
