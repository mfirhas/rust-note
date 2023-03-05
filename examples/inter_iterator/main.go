package main

import (
	"fmt"
)

func main() {
	fmt.Println(fullname("Muhammad")("Fathir")("Irhas"))
}

func fullname(firstname string) func(string) func(string) string {
	return func(middelname string) func(string) string {
		return func(lastname string) string {
			return firstname + " " + middelname + " " + lastname
		}
	}
}
