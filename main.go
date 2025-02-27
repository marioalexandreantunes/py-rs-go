package main

import (
	"fmt"
	"time"
)

func complexCalculation(n uint64) uint64 {
	var total uint64 = 0
	for i := uint64(0); i < n; i++ {
		total += i * i
		if i%2 == 0 {
			total += i
		} else {
			total -= i
		}
	}
	return total
}

func main() {
	n := uint64(100_000_000)

	start := time.Now()
	result := complexCalculation(n)
	duration := time.Since(start)

	fmt.Println("Go Results:")
	fmt.Printf("Result: %d\n", result)
	fmt.Printf("Time taken: %.4f seconds\n", duration.Seconds())
}