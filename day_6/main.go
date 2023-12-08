package main

import (
	"fmt"
	"io"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type data struct {
	time         int
	bestDistance int
}

func main() {
	f, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	content, err := io.ReadAll(f)
	if err != nil {
		log.Fatal(err)
	}

	races, err := parseInput(string(content))
	if err != nil {
		panic(err)
	}

	waysToWin := 1
	for _, race := range races {
		waysToWin *= race.getNumWaysToWin()
	}

	fmt.Println(waysToWin)
}

func (rd *data) getNumWaysToWin() int {
	a := -float64(1)
	b := float64(rd.time)
	c := -float64(rd.bestDistance)
	xMin := int(math.Floor((-b+math.Sqrt((math.Pow(b, float64(2)))-(4*a*c)))/(2*a))) + 1
	xMax := int(math.Ceil((-b-math.Sqrt((math.Pow(b, float64(2)))-(4*a*c)))/(2*a))) - 1
	return (xMax - xMin + 1)
}

func parseInput(fileContents string) ([]data, error) {
	racesData := []data{}
	lines := strings.Split(fileContents, "\n")
	times := strings.Fields(lines[0])
	distances := strings.Fields(lines[1])

	for i := 1; i < len(times); i++ {
		raceTime, err := strconv.Atoi(times[i])
		if err != nil {
			return []data{}, err
		}

		raceBestDistance, err := strconv.Atoi(distances[i])
		if err != nil {
			return []data{}, err
		}

		raceData := data{
			time:         raceTime,
			bestDistance: raceBestDistance,
		}
		racesData = append(racesData, raceData)
	}

	return racesData, nil
}
