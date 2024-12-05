package days

import (
	"bufio"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

func Day5(file *os.File, part string) {
	if part == "1" {
		day5_part1(file)
		return
	}
	day5_part2(file)
}

func parseInput(file *os.File) (map[int][]int, [][]int) {
	rules := map[int][]int{}
	updates := [][]int{}

	scanner := bufio.NewScanner(file)

	doneWithRules := false

	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			doneWithRules = true
			continue
		}

		if !doneWithRules {
			items := strings.Split(line, "|")

			x, err := strconv.Atoi(items[0])
			if err != nil {
				panic(err)
			}
			y, err := strconv.Atoi(items[1])
			if err != nil {
				panic(err)
			}

			_, ok := rules[x]

			if !ok {
				rules[x] = []int{}
			}

			rules[x] = append(rules[x], y)
			continue
		}

		var items []int
		for _, item := range strings.Split(line, ",") {
			v, err := strconv.Atoi(item)
			if err != nil {
				panic(err)
			}
			items = append(items, v)
		}

		updates = append(updates, items)
	}

	return rules, updates
}

// section one

// page ordering rules
// X|Y
// X before y

// section two
//

func day5_part1(file *os.File) {
	rules, updates := parseInput(file)

	midPoints := []int{}

	for _, update := range updates {

		updateLen := len(update)

		isValidUpdate := true

	updateCheck:
		for i, r := range update {
			item := rules[r]
			for _, rule := range item {
				if slices.Contains(update[:i], rule) {
					isValidUpdate = false
					break updateCheck
				}
			}
		}

		if !isValidUpdate {
			continue
		}

		mid := updateLen / 2
		midPoints = append(midPoints, update[mid])
	}

	var d int
	for _, i := range midPoints {
		d += i
	}

	log.Printf("Middles of order updates: %d", d)
}
func day5_part2(file *os.File) {
	rules, updates := parseInput(file)
	midPoints := []int{}

	for _, update := range updates {

		updateLen := len(update)
		wasFixed := false

		idx := 0
		l := len(update)
		for idx < l {
			value := update[idx]

			r, ok := rules[value]

			if !ok {
				idx++
				continue
			}

			for _, n := range r {
				subset := update[:idx]

				foundIdx := slices.Index(subset, n)
				if foundIdx != -1 {
					wasFixed = true
					offset := idx - (idx - foundIdx)
					temp := update[offset]
					update[offset] = update[idx]
					update[idx] = temp
					idx -= (idx - foundIdx)
				}
			}

			idx++
		}

		if !wasFixed {
			continue
		}

		mid := updateLen / 2
		midPoints = append(midPoints, update[mid])
	}

	var d int
	for _, i := range midPoints {
		d += i
	}

	log.Printf("Middles of order updates: %d", d)
}

// 97 before 13,29,47,75
// 75 before 13,29,47,
// 29 before 13
// 47 before 13,29

// 97,13,75,29,47 -> 97,75,47,29,13
// 97,75,13,29,47
// NOOP
// 97,75,29,13,47
//97,75,29,47,13
//97,75,47,29,13
