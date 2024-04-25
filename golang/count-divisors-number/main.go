package main

import "fmt"

func main() {
	a := Divisors(30)

	fmt.Println(a)
}

func Divisors(n int) (count int) {
	for v := 1; v <= n; v++ {
		if v == 0 {
			continue
		}
		if n%v == 0 {
			count = count + 1
		}
	}
	return
}
