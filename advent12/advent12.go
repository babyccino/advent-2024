package advent12

import (
	"advent/position"
	"bufio"
	"fmt"
	"log"
	"os"
)

const useLarge bool = true

type Row = []byte
type Arr = []Row

func getInput() (Arr, position.Position) {
	var file *os.File
	defer file.Close()

	if useLarge {
		openFile, err := os.Open("./advent12/large.txt")
		if err != nil {
			log.Fatal(err)
		}
		file = openFile
	} else {
		openFile, err := os.Open("./advent12/small.txt")
		if err != nil {
			log.Fatal(err)
		}
		file = openFile
	}

	scanner := bufio.NewScanner(file)

	y := 0
	ret := make([][]byte, 0)
	var xLen int
	for scanner.Scan() {
		text := scanner.Text()
		xLen = len(text)

		arr := make(Row, xLen)
		for x, char := range text {
			arr[x] = byte(char)
		}
		y += 1

		ret = append(ret, arr)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return ret, position.Position{X: xLen, Y: y}
}

func printByteArray(arr Arr) {
	for _, row := range arr {
		println(string(row))
	}
}

type Visited = [][]bool
type Pos = position.Position

type Solution2 struct {
	arr     Arr
	dim     Pos
	visited Visited
}

func (solution *Solution2) dfs2(
	pos Pos,
	char byte,
) (perimeter int, area int) {
	if solution.visited[pos.Y][pos.X] {
		return 0, 0
	}
	solution.visited[pos.Y][pos.X] = true
	area += 1

	for _, dir := range position.StraightVecs {
		newPos, found := pos.AddInBounds(dir, solution.dim)
		newChar := solution.arr[newPos.Y][newPos.X]
		// fmt.Printf("current char: %s, newPos: %s, char %s \n", string(char), newPos.String(), string(newChar))

		// if it's a boundary or a different colour add a fence
		if !found || newChar != char {
			perimeter += 1
			continue
		}

		addPerim, addArea := solution.dfs2(newPos, char)
		perimeter += addPerim
		area += addArea
	}

	return perimeter, area
}

func P1() {
	arr, dim := getInput()
	println(len(arr))
	printByteArray(arr)

	visited := make(Visited, dim.Y)
	for y := range visited {
		visited[y] = make([]bool, dim.X)
	}

	solution := Solution2{arr, dim, visited}

	totalPerim := 0
	totalArea := 0

	total := 0
	for y, row := range arr {
		for x, char := range row {
			pos := Pos{X: x, Y: y}
			addPerim, addArea := solution.dfs2(pos, char)
			if addArea == 0 {
				continue
			}
			totalPerim += addPerim
			totalArea += addArea
			add := addPerim * addArea
			total += add
			pos.Print()
			fmt.Printf("char %s: %d * %d = %d\n", string(char), addPerim, addArea, add)
		}
	}
	fmt.Printf("%d %d %d\n", totalPerim, totalArea, total)
}

// p2

type PosVec struct {
	pos Pos
	dir position.Direction
}

type Sides struct {
	sideStarts map[PosVec]Pos
	sideEnds   map[PosVec]Pos
}

func newSides() Sides {
	m1 := make(map[PosVec]Pos)
	m2 := make(map[PosVec]Pos)
	return Sides{m1, m2}
}

type SideMap = map[byte]Sides
type Solution struct {
	arr     Arr
	dim     Pos
	visited Visited
}

func sideHelper(dir position.Direction) (Pos, Pos) {
	switch dir {
	case position.Up:
		return position.LeftVec, position.RightVec
	case position.Down:
		return position.RightVec, position.LeftVec
	case position.Left:
		return position.DownVec, position.UpVec
	case position.Right:
		return position.UpVec, position.DownVec
	}
	return Pos{}, Pos{}
}

func (solution *Solution) sideHelper(
	pos position.Position,
	dir position.Direction, sides Sides,
) {
	lAdd, rAdd := sideHelper(dir)
	l, found := pos.AddInBounds(lAdd, solution.dim)

	var start Pos
	var foundEnd bool
	if found {
		start, foundEnd = sides.sideEnds[PosVec{l, dir}]
	}

	r, found := pos.AddInBounds(rAdd, solution.dim)
	var end Pos
	var foundStart bool
	if found {
		end, foundStart = sides.sideStarts[PosVec{r, dir}]
	}

	if foundStart && foundEnd {
		sides.sideStarts[PosVec{start, dir}] = end
		sides.sideEnds[PosVec{end, dir}] = start

		delete(sides.sideEnds, PosVec{l, dir})
		delete(sides.sideStarts, PosVec{r, dir})
	} else if foundEnd {
		sides.sideStarts[PosVec{start, dir}] = pos
		sides.sideEnds[PosVec{pos, dir}] = start

		delete(sides.sideEnds, PosVec{l, dir})
	} else if foundStart {
		sides.sideStarts[PosVec{pos, dir}] = end
		sides.sideEnds[PosVec{end, dir}] = pos

		delete(sides.sideStarts, PosVec{r, dir})
	} else {
		sides.sideStarts[PosVec{pos, dir}] = pos
		sides.sideEnds[PosVec{pos, dir}] = pos
	}
}

func (solution *Solution) dfs(
	pos Pos,
	char byte,
	sides Sides,
) (area int) {
	if solution.visited[pos.Y][pos.X] {
		return 0
	}
	solution.visited[pos.Y][pos.X] = true
	area += 1

	for dir := position.Up; dir <= position.Right; dir += 1 {
		dirVec := position.DirectionToVec(dir)
		newPos, found := pos.AddInBounds(dirVec, solution.dim)
		newChar := solution.arr[newPos.Y][newPos.X]

		// if it's a boundary or a different colour add a fence
		if !found || newChar != char {
			solution.sideHelper(pos, dir, sides)
			continue
		}

		addArea := solution.dfs(newPos, char, sides)
		area += addArea
	}

	return area
}

func P2() {
	arr, dim := getInput()
	println(len(arr))
	printByteArray(arr)

	visited := make(Visited, dim.Y)
	for y := range visited {
		visited[y] = make([]bool, dim.X)
	}

	solution := Solution{arr, dim, visited}

	total := 0
	for y, row := range arr {
		for x, char := range row {
			pos := Pos{X: x, Y: y}
			if visited[y][x] {
				continue
			}

			sides := newSides()
			addArea := solution.dfs(pos, char, sides)
			if addArea == 0 {
				continue
			}
			sideCount := len(sides.sideStarts)

			add := sideCount * addArea
			total += add
			fmt.Printf("char %s: %d * %d = %d\n", string(char), sideCount, addArea, add)
		}
	}

	fmt.Printf("%d\n", total)
}
