package advent1

import (
	"fmt"
)

func main() {
	arr1 := [...]int{}
	arr2 := [...]int{}

	count := make(map[int]int)
	for _, el := range arr2 {
		count[el] = count[el] + 1
	}

	total := 0
	for _, el := range arr1 {
		res, found := count[el]
		if found {
			total += el * res
		}
	}

	fmt.Printf("%d\n", total)
}
