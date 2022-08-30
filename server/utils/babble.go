package utils

import (
	"strings"

	"github.com/tjarratt/babble"
)

func getRandomWordsFromBabble(count int) ([]string, error) {
	babbler := babble.NewBabbler()
	babbler.Count = count
	babbler.Separator = " "
	words := babbler.Babble()
	return strings.Split(words, " "), nil
}
