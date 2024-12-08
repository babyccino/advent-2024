package advent2

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func abs(num int) int {
	if num < 0 {
		return -1 * num
	}
	return num
}

func removeOne(arr []int, index int) []int {
	newArr := make([]int, len(arr)-1)
	copy(newArr, arr[0:index])
	copy(newArr[index:], arr[index+1:])
	return newArr
}

var madeSafe []int

func isSafeArr(arr []int, remove bool) bool {
	if len(arr) == 1 {
		return true
	}
	increasing := arr[1] > arr[0]
	for i, el := range arr {
		if i == 0 {
			continue
		}

		before := arr[i-1]
		if before == el || (el > before) != increasing || abs(before-el) > 3 {
			if !remove {
				return false
			}

			newArr := removeOne(arr, i)
			newRes := isSafeArr(newArr, false)
			if newRes {
				return newRes
			}

			newArr = removeOne(arr, i-1)
			newRes = isSafeArr(newArr, false)
			return newRes
		}
	}
	return true
}

func isSafe(line string) bool {
	strs := strings.Split(line, " ")
	if len(strs) == 0 {
		log.Fatal("no nums in string: ", line)
	}

	arr := make([]int, 0, len(strs))
	for _, str := range strs {
		num, err := strconv.Atoi(str)
		if err != nil {
			log.Fatal(err)
		}
		arr = append(arr, num)
	}

	return isSafeArr(arr, true)
}

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	madeSafe = make([]int, 0)

	total := 0
	for scanner.Scan() {
		text := scanner.Text()
		res := isSafe(text)
		if res {
			total += 1
		}
	}

	fmt.Printf("total: %d\n", total)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
