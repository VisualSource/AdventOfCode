package days

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
	utils "visualsource/advent/utils"
)

func Day2(file *os.File, part string) {
	if part == "1" {
		day_2_part1(file)
	} else {
		day_2_part2(file)
	}
}

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}

func day_2_part1(file *os.File) {
	scanner := bufio.NewScanner(file)
	safeReports := 0

	for scanner.Scan() {
		line := scanner.Text()

		var data []int
		for _, v := range strings.Fields(line) {
			num, err := strconv.Atoi(v)
			if err != nil {
				panic(err)
			}
			data = append(data, num)
		}

		reportLen := len(data) - 1

		status := "safe"
		dir := 0

		idx := 0
		for {
			diff := data[idx+1] - data[idx]
			value := abs(diff)

			if value < 0 || value > 3 {
				status = "unsafe"
				break
			}

			if diff == 0 {
				// value is not increasing or decrease switch is unsafe
				status = "unsafe"
				break
			} else if diff > 0 {
				// increase
				if dir == 0 {
					dir = 1
				} else if dir != 1 {
					status = "unsafe"
					break
				}
			} else {
				if dir == 0 {
					dir = -1
					continue
				} else if dir != -1 {
					status = "unsafe"
					break
				}
			}

			idx += 1

			if idx >= reportLen {
				break
			}
		}

		if status == "safe" {
			safeReports += 1
		}
	}

	log.Printf("Safe reports: %d", safeReports)
}
func day_2_part2(file *os.File) {
	scanner := bufio.NewScanner(file)
	safeReports := 0

	lineIdx := 0
	for scanner.Scan() {
		line := scanner.Text()

		lineIdx += 1

		//log.Printf("++++ START %d ++++", lineIdx)

		var data []int
		for _, v := range strings.Fields(line) {
			num, err := strconv.Atoi(v)
			if err != nil {
				panic(err)
			}
			data = append(data, num)
		}

		status := "safe"
		dir := 0
		idx := 0
		usedDampener := false
		reportLen := len(data) - 1

		removeAndReset := func(idxToRemove int) {
			log.Printf("Removing index %d on line %d with value of %d", idx, lineIdx, data[idxToRemove])
			usedDampener = true
			data = utils.RemoveElement(data, idxToRemove)
			reportLen = len(data) - 1
			idx = 0
			dir = 0
		}

		for {
			if idx+1 > reportLen {
				break
			}
			diff := data[idx+1] - data[idx]
			value := abs(diff)

			//log.Printf("Diff %d Abs %d", diff, value)

			if value < 0 || value > 3 {
				if usedDampener {
					status = "unsafe"
					break
				}
				removeAndReset(idx)
				continue
			}

			if diff == 0 {
				if usedDampener {
					// value is not increasing or decrease switch is unsafe
					status = "unsafe"
					break
				}
				// can ignore error
				removeAndReset(idx)
				continue
			} else if diff > 0 {
				// increase
				if dir == 0 {
					dir = 1
				} else if dir != 1 {
					if usedDampener {
						status = "unsafe"
						break
					}
					removeAndReset(idx)
					continue
				}
			} else {
				if dir == 0 {
					dir = -1
				} else if dir != -1 {
					if usedDampener {
						status = "unsafe"
						break
					}
					removeAndReset(idx)
					continue
				}
			}

			idx += 1

			if idx >= reportLen {
				break
			}
		}

		//log.Printf("Status %s", status)
		if status == "safe" {
			safeReports += 1
		}

		//log.Print("++++ END ++++")
	}

	log.Printf("Safe reports: %d", safeReports)
}
