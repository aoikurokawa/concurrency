package waitgroup

import (
	"fmt"
	"sync"
	"time"
)

func WaitGroupFunction() {
	var wg sync.WaitGroup

	wg.Add(1)
	go func() {
		defer wg.Done()
		fmt.Println("1st goroutine sleeping...")
		time.Sleep(1)
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		fmt.Println("2nd goroutine sleeping...")
		time.Sleep(1)
	}()

	wg.Wait()
	fmt.Println("All goroutines complete.")
}
