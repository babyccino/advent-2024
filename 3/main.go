package main

import (
	"fmt"
	"regexp"
	"strconv"
)

var reg *regexp.Regexp

func parse(str string) int {
	reg = regexp.MustCompile(`mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(\)|do\(\)`)
	matches := reg.FindAllStringSubmatch(str, -1)
	res := 0
	enabled := true
	for _, match := range matches {
		if match[0] == "do()" {
			enabled = true
			continue
		}
		if match[0] == "don't()" {
			enabled = false
			continue
		}
		if !enabled {
			continue
		}

		arg1, err := strconv.Atoi(match[1])
		if err != nil {
			panic("ahhhh")
		}
		arg2, err := strconv.Atoi(match[2])
		if err != nil {
			panic("ahhhh")
		}
		res += arg1 * arg2
	}
	return res
}

func main() {
	str := `
	xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
	`
	fmt.Printf("%v\n", parse(str))
}
