package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"strconv"
	"strings"
	utils "visualsource/advent/utils"
)

const DAY = 1

var part = flag.String("part", "1", "set part to run")

func main() {
	file := utils.Load(DAY)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var leftList []int
	var rightList []int

	for scanner.Scan() {
		text := scanner.Text()

		fields := strings.Fields(text)

		a, err := strconv.Atoi(fields[0])
		if err != nil {
			panic(err)
		}

		leftList = append(leftList, a)

		b, err := strconv.Atoi(fields[1])
		if err != nil {
			panic(err)
		}

		rightList = append(rightList, b)
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	if *part == "1" {
		part1(leftList, rightList)
	} else {
		part2(leftList, rightList)
	}
}

func remove(s []int, i int) []int {
	if i >= len(s) || i < 0 {
		panic(fmt.Sprintf("Index is out of range. Index is %d with slice length %d", i, len(s)))
	}
	s[i] = s[0]
	return s[1:]
}

func minValue(list []int) (int, int) {
	idx := 0
	min := list[idx]
	for i, v := range list {
		if v < min {
			min = v
			idx = i
		}
	}

	return idx, min
}

func part1(left []int, right []int) {

	distence := 0

	for {
		if len(left) <= 0 || len(right) <= 0 {
			break
		}

		leftIdx, leftValue := minValue(left)
		rightIdx, rightValue := minValue(right)

		diff := max(leftValue, rightValue) - min(leftValue, rightValue)

		left = remove(left, leftIdx)
		right = remove(right, rightIdx)

		distence += diff

	}

	log.Printf("Part 1: %d\n", distence)
}
func part2(left []int, right []int) {
	similarity := 0

	seen := map[int]int{}

	for _, current := range left {

		calulatedSimilarity, exists := seen[current]
		if exists {
			similarity += calulatedSimilarity
			continue
		}

		count := 0
		for _, r := range right {
			if r == current {
				count += 1
			}
		}

		sim := current * count
		seen[current] = sim
		similarity += sim
	}

	log.Printf("Part 2: %d", similarity)
}
