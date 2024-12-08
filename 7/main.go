package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"slices"
	"strconv"
)

type Equation struct {
	ans      int
	args     []int
	original string
}

const useLarge = true

func getInput() []Equation {
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

	reg := regexp.MustCompile(`[0-9]+`)
	arr := make([]Equation, 0)
	for scanner.Scan() {
		text := scanner.Text()
		matches := reg.FindAllString(text, -1)

		if len(matches) < 2 {
			panic("ahhhhh equation arr too short")
		}

		ans, err := strconv.Atoi(matches[0])
		if err != nil {
			panic("ahhhhh no int")
		}

		args := make([]int, len(matches)-1)
		for i, match := range matches[1:] {
			arg, err := strconv.Atoi(match)
			if err != nil {
				panic("ahhhhh no int")
			}
			args[i] = arg
		}
		slices.Reverse(args)

		eq := Equation{ans, args, text}
		// println(eq.ans)
		// fmt.Printf("%v\n", eq.args)
		arr = append(arr, eq)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return arr
}

func perms(eq Equation) int {
	return _perms(eq.args, eq.ans)
}
func _perms(args []int, looking int) int {
	if len(args) == 0 {
		return 0
	}

	arg := args[0]

	if len(args) == 1 {
		if looking == arg {
			return 1
		} else {
			return 0
		}
	}

	total := 0
	if looking-arg > 0 {
		total += _perms(args[1:], looking-arg)
	}
	if looking%arg == 0 {
		total += _perms(args[1:], looking/arg)
	}

	return total
}

func p1() {
	arr := getInput()
	total := 0
	for _, eq := range arr {
		res := perms(eq)
		if res > 0 {
			fmt.Printf("str %s gives %d\n", eq.original, res)
			total += eq.ans
		}
	}
	fmt.Printf("\ntotal: %d\n", total)
}

func main() {
	p1()
}
