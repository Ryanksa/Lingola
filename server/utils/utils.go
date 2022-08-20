package utils

import (
	"errors"
	"log"
	"os"
	"regexp"
	"strings"

	"github.com/joho/godotenv"
	"github.com/tjarratt/babble"
)

func LoadEnv() {
	projectDirName := "server"
	projectName := regexp.MustCompile(`^(.*` + projectDirName + `)`)
	currentWorkDirectory, _ := os.Getwd()
	rootPath := projectName.Find([]byte(currentWorkDirectory))

	err := godotenv.Load(string(rootPath) + `/.env`)

	if err != nil {
		log.Fatalf("Error loading .env file")
	}
}

func GetRandomWords(count int) ([]string, error) {
	if count <= 0 {
		return []string{}, errors.New("count needs to be greater than 0")
	}
	babbler := babble.NewBabbler()
	babbler.Count = count
	babbler.Separator = " "
	words := babbler.Babble()
	return strings.Split(words, " "), nil
}

func GetWordDefinition(word string, channel chan string) {
	definition, _ := getWordnikDefinition(word)
	channel <- definition
}
