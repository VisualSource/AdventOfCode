package main

import (
	"flag"
	"log"
	"time"
	days "visualsource/advent/days"
	utils "visualsource/advent/utils"
)

var day = flag.Int("day", 0, "Override the day to run")
var part = flag.String("part", "1", "set part to run")

func main() {

	dayToRun := time.Now().Day()
	if *day != 0 {
		dayToRun = *day
	}

	log.Printf("Running day %d, part %s", dayToRun, *part)

	file := utils.Load(dayToRun)
	defer file.Close()

	switch dayToRun {
	case 1:
		days.Day1(file, *part)
	case 2:
		days.Day2(file, *part)
	case 3:
		days.Day3(file, *part)
	case 4:
		days.Day4(file, *part)
	case 5:
		days.Day5(file, *part)

	default:
		panic("Invalid day given")
	}

}
