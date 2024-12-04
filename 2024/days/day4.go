package days

import (
	"bufio"
	"log"
	"os"
	"slices"
	"strings"
)

func filter[T any](ss []T, test func(T) bool) (ret []T) {
	for _, s := range ss {
		if test(s) {
			ret = append(ret, s)
		}
	}
	return
}

func Day4(file *os.File, part string) {
	if part == "1" {
		day4_part1(file)
		return
	}
	day4_part2(file)
}

func day4_part1(file *os.File) {
	scanner := bufio.NewScanner(file)

	forwardCheck := []string{"X", "M", "A", "S"}

	var items [][]string

	for scanner.Scan() {
		line := scanner.Text()
		items = append(items, strings.Split(line, ""))
	}

	columnHeight := len(items)
	rowWidth := len(items[0])

	found := 0

	//runtime.Breakpoint()

	for y, xLine := range items {
		for x, yLine := range xLine {
			if yLine != "X" {
				continue
			}

			founds := []bool{
				true, // up
				true, // down
				true, // left
				true, // right
				true, // down-right
				true, // up-left
				true, // down-left
				true, // up-right
			}

			for i, v := range forwardCheck {
				// horizontal right
				if (x+i >= rowWidth) || v != items[y][x+i] {
					founds[0] = false
				}
				// horizontal left
				if (x-i < 0) || v != items[y][x-i] {
					founds[1] = false
				}

				// vertical: down
				if (y+i >= columnHeight) || v != items[y+i][x] {
					founds[2] = false
				}
				// vertical: up
				if (y-i < 0) || v != items[y-i][x] {
					founds[3] = false
				}

				// diagonal: down-right
				if (y+i >= columnHeight) || (x+i >= rowWidth) || v != items[y+i][x+i] {
					founds[4] = false
				}
				// diagonal: up-left
				if (y-i < 0) || (x-i < 0) || v != items[y-i][x-i] {
					founds[5] = false
				}

				// diagonal: down-left
				if (y+i >= columnHeight) || (x-i < 0) || v != items[y+i][x-i] {
					founds[6] = false
				}

				// diagonal: up-right
				if (y-i < 0) || (x+i >= rowWidth) || v != items[y-i][x+i] {
					founds[7] = false
				}
			}

			for _, f := range founds {
				if f {
					found++
				}
			}
		}
	}

	log.Printf("Found %d", found)
}

func day4_part2(file *os.File) {
	scanner := bufio.NewScanner(file)

	valid := []string{"M", "S"}

	var items [][]string

	for scanner.Scan() {
		line := scanner.Text()
		items = append(items, strings.Split(line, ""))
	}

	columnHeight := len(items)
	rowWidth := len(items[0])

	found := 0

	//runtime.Breakpoint()

	for y, row := range items {
		for x, item := range row {
			if item != "A" {
				continue
			}

			// check to see if we the A is on a edge
			if (y-1 < 0) || (x-1 < 0) || (x+1 >= rowWidth) || (y+1 >= columnHeight) {
				continue
			}

			// check that diags are not the same
			if (items[y-1][x-1] == items[y+1][x+1]) || (items[y+1][x-1] == items[y-1][x+1]) {
				continue
			}

			// check if values are 'M' | 'S'
			if !slices.Contains(valid, items[y-1][x-1]) || !slices.Contains(valid, items[y+1][x+1]) ||
				!slices.Contains(valid, items[y+1][x-1]) || !slices.Contains(valid, items[y-1][x+1]) {
				continue
			}

			found++
		}

	}

	log.Printf("Found %d", found)
}
