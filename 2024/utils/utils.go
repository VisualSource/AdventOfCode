package utils

import (
	"flag"
	"fmt"
	"log"
	"os"
	"path/filepath"
)

var mode = flag.String("mode", "test", "the mode in which to run: test | prod")
var dir = flag.String("dir", "", "current dir")

func Load(day int) *os.File {
	flag.Parse()
	log.SetFlags(0)

	folder := "examples"
	if *mode == "prod" {
		folder = "inputs"
	}

	fileName := fmt.Sprintf("day%d.txt", day)

	filePath := filepath.Join(*dir, "data", folder, fileName)

	file, err := os.OpenFile(filePath, os.O_RDONLY, 466)
	if err != nil {
		panic(err)
	}

	return file
}
