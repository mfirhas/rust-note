package main

import (
	"fmt"
)

func main() {
	var str = []int{1, 2, 3}
	fmt.Println("before", str)
	Slice(str)
	// Str(str)
	fmt.Println("after", str)

}

func Str(str string) {
}

func Slice(s []int) {
	// s[0] = 10
	fmt.Println(s)
	s = []int{3, 2, 1}
	fmt.Println(s)
}
