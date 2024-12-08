package advent4

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

const xmasLen = 4

type Direction int

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

var tracker [][]byte

var directionArray = [...]([2]int){
	{1, 1},
	{1, -1},
	{-1, -1},
	{-1, 1},
	{-1, 0},
	{1, 0},
	{0, -1},
	{0, 1},
}

func directionToVec(dir Direction) (int, int) {
	el := directionArray[dir]
	return el[0], el[1]
}

// Debug
func addVec(i int, j int, dir Direction, scalar int, char byte) {
	vecY, vecX := directionToVec(dir)
	y := i + (vecY * scalar)
	x := j + (vecX * scalar)
	tracker[y][x] = char
}

func checkVec2(lines []string, i int, j int, dir Direction, scalar int, char byte) bool {
	vecY, vecX := directionToVec(dir)
	y := i + (vecY * scalar)
	x := j + (vecX * scalar)
	return lines[y][x] == char
}

func searchInnerInner2(lines []string, i int, j int, dir Direction) int {
	if checkVec2(lines, i, j, dir, 1, 'M') &&
		checkVec2(lines, i, j, dir, 2, 'A') &&
		checkVec2(lines, i, j, dir, 3, 'S') {
		// Debug
		// addVec(i, j, dir, 0, 'X')
		// addVec(i, j, dir, 1, 'M')
		// addVec(i, j, dir, 2, 'A')
		// addVec(i, j, dir, 3, 'S')
		return 1
	}
	return 0
}

func searchInner2(lines []string, i int, j int) int {
	if lines[i][j] != 'X' {
		return 0
	}

	top := i >= xmasLen-1
	bottom := i+xmasLen <= len(lines)
	right := j+xmasLen <= len(lines[0])
	left := j >= xmasLen-1

	total := 0

	if top {
		total += searchInnerInner2(lines, i, j, Up)
		if left {
			total += searchInnerInner2(lines, i, j, UpLeft)
		}
		if right {
			total += searchInnerInner2(lines, i, j, UpRight)
		}
	}
	if bottom {
		total += searchInnerInner2(lines, i, j, Down)
		if left {
			total += searchInnerInner2(lines, i, j, DownLeft)
		}
		if right {
			total += searchInnerInner2(lines, i, j, DownRight)
		}
	}

	if left {
		total += searchInnerInner2(lines, i, j, Left)
	}
	if right {
		total += searchInnerInner2(lines, i, j, Right)
	}

	return total
}

func search2(lines []string) int {
	total := 0
	for i := 0; i < len(lines); i++ {
		for j := 0; j < len(lines[0]); j++ {
			total += searchInner2(lines, i, j)
		}
	}
	return total
}

func checkVec(lines []string, i int, j int, dir Direction, char byte) bool {
	vecY, vecX := directionToVec(dir)
	y := i + vecY
	x := j + vecX
	return lines[y][x] == char
}

func searchInnerInner(lines []string, i int, j int, dir Direction) bool {
	if dir == DownRight {
		return checkVec(lines, i, j, UpLeft, 'M') && checkVec(lines, i, j, DownRight, 'S')
	}
	if dir == DownLeft {
		return checkVec(lines, i, j, UpRight, 'M') && checkVec(lines, i, j, DownLeft, 'S')
	}
	if dir == UpLeft {
		return checkVec(lines, i, j, DownRight, 'M') && checkVec(lines, i, j, UpRight, 'S')
	}
	if dir == UpRight {
		return checkVec(lines, i, j, DownLeft, 'M') && checkVec(lines, i, j, UpRight, 'S')
	}
	return false
}

func searchInner(lines []string, i int, j int) int {
	if lines[i][j] != 'A' {
		return 0
	}

	if i-1 < 0 || i+1 >= len(lines) || j-1 < 0 || j+1 >= len(lines[0]) {
		return 0
	}

	if searchInnerInner(lines, i, j, DownRight) &&
		(searchInnerInner(lines, i, j, DownLeft) ||
			searchInnerInner(lines, i, j, UpRight)) {
		return 1
	}
	if searchInnerInner(lines, i, j, DownLeft) && searchInnerInner(lines, i, j, UpLeft) {
		return 1
	}
	if searchInnerInner(lines, i, j, UpLeft) && searchInnerInner(lines, i, j, UpRight) {
		return 1
	}

	return 0
}

func search(lines []string) int {
	total := 0
	for i := 0; i < len(lines); i++ {
		for j := 0; j < len(lines[0]); j++ {
			total += searchInner(lines, i, j)
		}
	}
	return total
}

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	lines := make([]string, 0)
	tracker = make([][]byte, 0)
	for scanner.Scan() {
		text := scanner.Text()
		lines = append(lines, text)
		app := make([]byte, len(text))
		for i := range app {
			app[i] = '.'
		}
		tracker = append(tracker, app)
	}

	total := search(lines)
	fmt.Printf("total: %d\n", total)

	// for _, line := range tracker {
	// 	println(string(line))
	// }
	// println("")

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
