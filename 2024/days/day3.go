package days

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"regexp"
	"strings"
)

func Day3(file *os.File, part string) {

	if part == "1" {
		day3_part1(file)
		return
	}
	day3_part2(file)
}

func day3_part1(file *os.File) {
	reader := bufio.NewReader(file)

	builder := strings.Builder{}
	for {
		buf := make([]byte, 1024)

		_, err := reader.Read(buf)
		if err != nil {
			if err == io.EOF {
				break
			} else {
				panic(err)
			}
		}

		builder.Write(buf)
	}

	//cmd := []rune("mul(")
	var result int64 = 0
	re := regexp.MustCompile(`mul\(\d{1,3},\d{1,3}\)`)

	for _, v := range re.FindAll([]byte(builder.String()), -1) {
		var (
			a, b int64
		)

		_, err := fmt.Sscanf(string(v), "mul(%d,%d)", &a, &b)
		if err != nil {
			panic(err)
		}

		result += (a * b)
	}

	/*for {
		if err := consumeRune(reader, 'm', false); err != nil {
			if isFailedToFindError(err) {
				continue
			} else if isEof(err, false) {
				break
			} else {
				panic(err)
			}
		}

		// we found m, try to parse command

		if err := consumeCmd(reader, cmd); err != nil {
			if isFailedToFindError(err) {
				continue
			} else if isEof(err, false) {
				break
			} else {
				panic(err)
			}
		}

		numA, err := parseNumber(reader)
		if err != nil {
			if isEof(err, false) {
				break
			} else if isFailedToFindError(err) {
				continue
			} else {
				panic(err)
			}
		}

		err = consumeRune(reader, ',', true)
		if err != nil {
			if isEof(err, false) {
				break
			} else if isFailedToFindError(err) {
				continue
			} else {
				panic(err)
			}
		}

		numB, err := parseNumber(reader)
		if err != nil {
			if isEof(err, false) {
				break
			} else if isFailedToFindError(err) {
				continue
			} else {
				panic(err)
			}
		}

		err = consumeRune(reader, ')', true)
		if err != nil {
			if isEof(err, false) {
				break
			} else if isFailedToFindError(err) {
				continue
			} else {
				panic(err)
			}
		}

		result += (numA * numB)
	}*/

	log.Printf("%d", result)
}

func day3_part2(file *os.File) {
	reader := bufio.NewReader(file)

	builder := strings.Builder{}
	for {
		buf := make([]byte, 1024)

		_, err := reader.Read(buf)
		if err != nil {
			if err == io.EOF {
				break
			} else {
				panic(err)
			}
		}

		builder.Write(buf)
	}

	var result int64 = 0
	re := regexp.MustCompile(`(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don\'t\(\))`)

	enabled := true

	for _, v := range re.FindAll([]byte(builder.String()), -1) {
		value := string(v)

		if value == "do()" {
			enabled = true
		} else if value == "don't()" {
			enabled = false
		} else {
			if !enabled {
				continue
			}
			var (
				a, b int64
			)

			_, err := fmt.Sscanf(string(v), "mul(%d,%d)", &a, &b)
			if err != nil {
				panic(err)
			}

			result += (a * b)
		}
	}

	log.Printf("%d", result)
}
