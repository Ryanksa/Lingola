package utils

import (
	"errors"
	"log"
	"os"
	"regexp"

	"github.com/joho/godotenv"
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

func GetRandomWords(num int) ([]string, error) {
	if num <= 0 {
		return []string{}, errors.New("num needs to be greater than 0")
	}

	var words []string
	var err error
	if os.Getenv("RANDOMIZER") == "wordnik" {
		words, err = getRandomWordsFromWordnik(num)
	} else if os.Getenv("RANDOMIZER") == "babble" {
		words, err = getRandomWordsFromBabble(num)
	} else {
		err = errors.New("missing RANDOMIZER env variable")
	}

	if err != nil {
		return []string{}, err
	}
	return words, nil
}

func GetWordDefinition(word string, channel chan string) {
	definition, _ := getWordnikDefinition(word)
	channel <- definition
}
