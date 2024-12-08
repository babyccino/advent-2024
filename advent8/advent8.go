package advent8

import (
	"bufio"
	"fmt"
	"log"
	"os"

	"advent/position"
)

type Row = []position.Position
type Map = map[rune]Row

const useLarge = true

func getInput() (Map, position.Position, Hash, [][]byte) {
	var file *os.File
	defer file.Close()

	if useLarge {
		openFile, err := os.Open("./advent8/large.txt")
		if err != nil {
			log.Fatal(err)
		}
		file = openFile
	} else {
		openFile, err := os.Open("./advent8/small.txt")
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

			pos := position.Position{X: x, Y: y}
			all[pos] = Taken
			arr = append(arr, pos)
			res[char] = arr
		}
		y += 1
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return res, position.Position{X: xLen, Y: y}, all, original
}

type State = byte

const Taken State = 0

type Hash = map[position.Position]State

func anti(pos1 position.Position, pos2 position.Position, max *position.Position) (*position.Position, *position.Position) {
	deltaX := pos2.X - pos1.X
	deltaY := pos2.Y - pos1.Y

	var ret1 *position.Position
	var ret2 *position.Position

	x := pos1.X - deltaX
	y := pos1.Y - deltaY
	if x < max.X && y < max.Y && x >= 0 && y >= 0 {
		ret1 = &position.Position{X: x, Y: y}
	}

	x = pos2.X + deltaX
	y = pos2.Y + deltaY
	if x < max.X && y < max.Y && x >= 0 && y >= 0 {
		ret2 = &position.Position{X: x, Y: y}
	}

	return ret1, ret2
}

var Empty struct{}

func check2(hash Hash, arr Row, state State, max *position.Position) {
	if len(arr) < 2 {
		return
	}

	rest := arr[1:]
	pos1 := arr[0]
	for _, pos2 := range rest {
		anti1, anti2 := anti(pos1, pos2, max)

		if anti1 != nil {
			anti1.Print()
			_, found := hash[*anti1]
			if !found {
				hash[*anti1] = state
			}
		}
		if anti2 != nil {
			anti2.Print()
			_, found := hash[*anti2]
			if !found {
				hash[*anti2] = state
			}
		}

		println("")
	}

	check2(hash, rest, state, max)
}

func P1() {
	res, max, hash, original := getInput()
	for state, row := range res {
		check2(hash, row, byte(state), &max)
	}

	for pos, state := range hash {
		if state != Taken {
			original[pos.Y][pos.X] = '#'
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
	max := position.Position{X: 10, Y: 10}
	anti1, anti2 := anti(position.Position{X: 0, Y: 0}, position.Position{X: 1, Y: 2}, &max)

	if anti1 == nil {
		println("no anti1")
	} else {
		fmt.Printf("x1, y1: %d, %d\n", anti1.X, anti1.Y)
	}

	if anti2 == nil {
		println("no anti2")
	} else {
		fmt.Printf("x2, y2: %d, %d\n", anti2.X, anti2.Y)
	}
}

// P2

func antiMult(pos1 position.Position, pos2 position.Position, max *position.Position) []position.Position {
	diff := pos2.Diff(pos1)

	ret := make([]position.Position, 0)

	mult := 1
	for {
		newPos := pos2.AddMult(diff, mult)
		if newPos.X < max.X && newPos.Y < max.Y && newPos.X >= 0 && newPos.Y >= 0 {
			ret = append(ret, newPos)
		} else {
			break
		}
		mult += 1
	}

	mult = -1
	for {
		newPos := pos1.AddMult(diff, mult)
		if newPos.X < max.X && newPos.Y < max.Y && newPos.X >= 0 && newPos.Y >= 0 {
			ret = append(ret, newPos)
		} else {
			break
		}
		mult -= 1
	}

	return ret
}

func check(hash Hash, arr Row, state State, max *position.Position) {
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

func P2() {
	res, max, hash, original := getInput()
	for state, row := range res {
		check(hash, row, byte(state), &max)
	}

	for pos, state := range hash {
		if state != Taken {
			original[pos.Y][pos.X] = '#'
		}
	}

	for _, line := range original {
		println(string(line))
	}

	total := len(hash)
	fmt.Printf("total %d\n", total)
}
