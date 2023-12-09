package main

import (
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)

func main() {
	now := time.Now()
	content, err := parseInput("./input.txt")
	if err != nil {
		log.Fatalf("%v", err)
	}

	sum := 0
	lines := strings.Split(content, "\n")
	for _, l := range lines {
		seq := strings.Split(l, " ")
		sum += analyze(convertToInt(seq), 0)
	}
	fmt.Println(time.Since(now))
	fmt.Println(sum)
}

func parseInput(filename string) (string, error) {
	f, err := os.Open(filename)
	if err != nil {
		return "", err
	}
	defer f.Close()

	content, err := io.ReadAll(f)
	if err != nil {
		return "", err
	}
	return string(content), nil
}

func convertToInt(seq []string) (result []int64) {
	for i := range seq {
		n, err := strconv.ParseInt(seq[i], 0, 64)
		if err != nil {
			return nil
		}
		result = append(result, n)
	}
	return result
}

func analyze(seq []int64, i int) int {
	if i != 0 {
		zeroed := true
		for i := range seq {
			if seq[i] != 0 {
				zeroed = false
			}
		}
		if zeroed {
			return i
		}
	}
	support := []int64{}
	for j := range seq {
		if j < len(seq)-1 {
			support = append(support, seq[j+1]-seq[j])
			continue
		}
		return analyze(support, i+int(seq[len(seq)-1]))
	}
	return i
}
