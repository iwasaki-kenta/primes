package main

import (
	"os"
	"log"
	"io/ioutil"
	"strconv"
	"sync"
	"strings"
)

func processFile(primes *[]string, num int) {
	file, err := os.Open("data/primes" + strconv.Itoa(num+1) + ".txt")
	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()

	// Seek 75 bytes into the file.
	file.Seek(75, 0)

	text, err := ioutil.ReadAll(file)
	if err != nil {
		log.Fatal(err)
	}

	for read, x, y := 0, -1, 0; y < len(text); y++ {
		if text[y] >= '0' && text[y] <= '9' {
			if x == -1 {
				x = y
			}
		} else {
			if x != -1 {
				(*primes)[num*1000000+read] = string(text[x:y])
				read += 1
				x = -1
			}
		}
	}
}

func main() {
	primes := make([]string, 50000000, 50000000)

	wg := sync.WaitGroup{}
	for num := 0; num < 50; num++ {
		wg.Add(1)

		go func(num int) {
			defer wg.Done()
			processFile(&primes, num)
		}(num)
	}

	wg.Wait()

	ioutil.WriteFile("result.csv", []byte(strings.Join(primes, "\n")), 0644)

}
