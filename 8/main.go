package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Position struct {
	x int
	y int
}

func (pos *Position) print() {
	fmt.Printf("Position{x: %d, y: %d}\n", pos.x, pos.y)
}
func (pos *Position) add(other Position) Position {
	return Position{pos.x + other.x, pos.y + other.y}
}
func (pos *Position) addMult(other Position, mult int) Position {
	return Position{pos.x + mult*other.x, pos.y + mult*other.y}
}
func (pos *Position) diff(other Position) Position {
	return pos.addMult(other, -1)
}

type Row = []Position
type Map = map[rune]Row

const useLarge = true

func getInput() (Map, Position, Hash, [][]byte) {
	var file *os.File
	defer file.Close()

	if useLarge {
		openFile, err := os.Open("./large.txt")
		if err != nil {
			log.Fatal(err)
		}
		file = openFile
	} else {
		openFile, err := os.Open("./small.txt")
		if err != nil {
			log.Fatal(err)
		}
		file = openFile
	}

	scanner := bufio.NewScanner(file)

	res := Map{}
	y := 0
	original := make([][]byte, 0)
	all := Hash{}
	var xLen int
	for scanner.Scan() {
		text := scanner.Text()
		original = append(original, []byte(text))
		xLen = len(text)

		for x, char := range text {
			if char == '.' {
				continue
			}

			arr, found := res[char]
			if !found {
				arr = make(Row, 0)
			}

			pos := Position{x, y}
			all[pos] = Taken
			arr = append(arr, pos)
			res[char] = arr
		}
		y += 1
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return res, Position{xLen, y}, all, original
}

type State = byte

const Taken State = 0

type Hash = map[Position]State

func anti(pos1 Position, pos2 Position, max *Position) (*Position, *Position) {
	deltaX := pos2.x - pos1.x
	deltaY := pos2.y - pos1.y

	var ret1 *Position
	var ret2 *Position

	x := pos1.x - deltaX
	y := pos1.y - deltaY
	if x < max.x && y < max.y && x >= 0 && y >= 0 {
		ret1 = &Position{x, y}
	}

	x = pos2.x + deltaX
	y = pos2.y + deltaY
	if x < max.x && y < max.y && x >= 0 && y >= 0 {
		ret2 = &Position{x, y}
	}

	return ret1, ret2
}

var Empty struct{}

func check2(hash Hash, arr Row, state State, max *Position) {
	if len(arr) < 2 {
		return
	}

	rest := arr[1:]
	pos1 := arr[0]
	for _, pos2 := range rest {
		anti1, anti2 := anti(pos1, pos2, max)

		if anti1 != nil {
			anti1.print()
			_, found := hash[*anti1]
			if !found {
				hash[*anti1] = state
			}
		}
		if anti2 != nil {
			anti2.print()
			_, found := hash[*anti2]
			if !found {
				hash[*anti2] = state
			}
		}

		println("")
	}

	check2(hash, rest, state, max)
}

func p1() {
	res, max, hash, original := getInput()
	for state, row := range res {
		check2(hash, row, byte(state), &max)
	}

	for pos, state := range hash {
		if state != Taken {
			original[pos.y][pos.x] = '#'
		}
	}

	for _, line := range original {
		println(string(line))
	}

	total := 0
	for _, state := range hash {
		if state != Taken {
			total += 1
		}
	}

	fmt.Printf("total %d\n", total)
}

func testAnti() {
	max := Position{10, 10}
	anti1, anti2 := anti(Position{0, 0}, Position{1, 2}, &max)

	if anti1 == nil {
		println("no anti1")
	} else {
		fmt.Printf("x1, y1: %d, %d\n", anti1.x, anti1.y)
	}

	if anti2 == nil {
		println("no anti2")
	} else {
		fmt.Printf("x2, y2: %d, %d\n", anti2.x, anti2.y)
	}
}

// p2

func antiMult(pos1 Position, pos2 Position, max *Position) []Position {
	diff := pos2.diff(pos1)

	ret := make([]Position, 0)

	mult := 1
	for {
		newPos := pos2.addMult(diff, mult)
		if newPos.x < max.x && newPos.y < max.y && newPos.x >= 0 && newPos.y >= 0 {
			ret = append(ret, newPos)
		} else {
			break
		}
		mult += 1
	}

	mult = -1
	for {
		newPos := pos1.addMult(diff, mult)
		if newPos.x < max.x && newPos.y < max.y && newPos.x >= 0 && newPos.y >= 0 {
			ret = append(ret, newPos)
		} else {
			break
		}
		mult -= 1
	}

	return ret
}

func check(hash Hash, arr Row, state State, max *Position) {
	if len(arr) < 2 {
		return
	}

	rest := arr[1:]
	pos1 := arr[0]
	for _, pos2 := range rest {
		antis := antiMult(pos1, pos2, max)

		for _, anti := range antis {
			_, found := hash[anti]
			if !found {
				hash[anti] = state
			}
		}
	}

	check(hash, rest, state, max)
}

func p2() {
	res, max, hash, original := getInput()
	for state, row := range res {
		check(hash, row, byte(state), &max)
	}

	for pos, state := range hash {
		if state != Taken {
			original[pos.y][pos.x] = '#'
		}
	}

	for _, line := range original {
		println(string(line))
	}

	total := len(hash)
	fmt.Printf("total %d\n", total)
}

func main() {
	p2()
}
