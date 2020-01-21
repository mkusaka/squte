package main

import "fmt"

func main() {
	var chosen string

	for {
		fmt.Print("choose 1 or 2 or 3: ")
		fmt.Scan(&chosen)
		if chosen == "1" || chosen == "2" || chosen == "3" {
			break
		}
	}

	fmt.Printf("you choose %s", chosen)
}
