package main

import (
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"
)

func showFiles(files []string) []string {
	allFiles := make([]string, 0)
	c := 1
	for _, file := range files {
		if file != "folderStructure" && file != "README" {
			allFiles = append(allFiles, file)
			c += 1
		}
	}
	return allFiles
}

func foldersName(files []string) map[string]int {
	mk := make(map[string]int)
	i := 1
	for _, file := range files {

		str1 := strings.Split(file, ".")
		if str1[0] != "folderStructure" && file != "README" {
			mk[string(str1[0])] = i
			i += 1
		}

	}

	return mk
}

func createDirectory(foldername map[string]int) {
	fmt.Println("Creating folder...")
	time.Sleep(2 * time.Second)
	count := 1
	for dir, _ := range foldername {
		// time.Sleep(1 * time.Second)
		if _, err := os.Stat(dir); os.IsNotExist(err) {
			fmt.Println(count, dir)
			if err := os.MkdirAll(dir, os.ModePerm); err != nil {
				log.Fatal(err)
			}
		}
		count += 1
	}

}

func moveToDirectory(allFiles []string) {
	fmt.Println("Moveing to directory....")
	for _, dir := range allFiles {
		str1 := strings.Split(dir, ".")
		if str1[0] != dir {
			info, err := os.Stat(str1[0])
			b := !os.IsNotExist(err) && info.IsDir()
			if b {
				err := os.Rename("./"+dir, "./"+str1[0]+"/"+dir)
				if err != nil {
					log.Fatal(err)
				}
			}
		}

	}
}

func main() {

	files, err := filepath.Glob("*")
	if err != nil {
		log.Fatal(err)
	}

	allFiles := showFiles(files)

	fmt.Println("----------", allFiles)

	foldername := foldersName(files)

	fmt.Println(foldername)
	// time.Sleep(5 * time.Second)
	// Create folder for every file

	// createDirectory(foldername)
	// time.Sleep(5 * time.Second)

	defer moveToDirectory(allFiles)

}
