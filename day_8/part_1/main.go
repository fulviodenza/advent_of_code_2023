package main

import (
	"fmt"
	"io"
	"log"
	"os"
	"regexp"
	"strings"
)

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

func main() {
	content, err := parseInput("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(content, "\n")
	path := lines[2 : len(lines)-1]

	route := map[string][2]string{}
	for _, l := range path {
		s1, s2 := extractStrings(l)
		route[l[0:3]] = [2]string{s1, s2}
	}

	steps := followInstructions(lines, route)
	fmt.Printf("Steps to reach ZZZ: %d\n", steps)
}

func followInstructions(lines []string, route map[string][2]string) int {
	direction := lines[0]
	directionIterator := 0

	currentNode := "AAA"

	steps := 0
	curr := ""
	for {
		steps++
		if direction[directionIterator] == 'L' {
			curr = route[currentNode][0]
		} else {
			curr = route[currentNode][1]
		}

		if curr == "ZZZ" {
			break
		}
		currentNode = curr

		directionIterator = (directionIterator + 1) % len(direction)
	}
	return steps
}

func extractStrings(input string) (string, string) {
	re := regexp.MustCompile(`\(([^)]+)\)`)
	matches := re.FindStringSubmatch(input)

	if len(matches) < 2 {
		return "", ""
	}

	values := strings.Split(matches[1], ",")

	return strings.TrimSpace(values[0]), strings.TrimSpace(values[1])
}
