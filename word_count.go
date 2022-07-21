package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func read_confessions(fname string) *os.File {
	f, err := os.Open(fname)
	if err != nil {
		log.Fatal(err)
	}
	return f
}

func get_confessions_by_line(f *os.File) map[string]int {
	scanner := bufio.NewScanner(f)
	word_count := make(map[string]int)
	for scanner.Scan() {
		line_replacer := strings.NewReplacer(".", "", ",", "", ";", "", "?", "", "!", "", "a", "", "the", "")
		line := line_replacer.Replace(scanner.Text())
		words := strings.Fields(line)
		for _, word := range words {
			word_count[word]++
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	return word_count
}

func main() {
	var many_confessions *os.File
	many_confessions = read_confessions("confessions.txt")
	var counts map[string]int
	counts = get_confessions_by_line(many_confessions)
	for key, element := range counts {
		if element > 100 {
			fmt.Println(key, element)
		}
	}
}
