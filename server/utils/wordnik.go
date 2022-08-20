package utils

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"os"
)

func getWordnikDefinition(word string) (string, error) {
	url := fmt.Sprintf("https://api.wordnik.com/v4/word.json/%s/definitions?limit=1&includeRelated=false&useCanonical=false&includeTags=false&api_key=%s", word, os.Getenv("WORDNIK_API_KEY"))
	res, err := http.Get(url)
	if err != nil {
		return "", err
	}
	defer res.Body.Close()

	data := struct {
		WordnikDefinition []struct {
			ID               string        `json:"id"`
			PartOfSpeech     string        `json:"partOfSpeech"`
			AttributionText  string        `json:"attributionText"`
			SourceDictionary string        `json:"sourceDictionary"`
			Text             string        `json:"text"`
			Sequence         string        `json:"sequence"`
			Score            int           `json:"score"`
			Word             string        `json:"word"`
			AttributionURL   string        `json:"attributionUrl"`
			WordnikURL       string        `json:"wordnikUrl"`
			Citations        []interface{} `json:"citations"`
			ExampleUses      []interface{} `json:"exampleUses"`
			Labels           []interface{} `json:"labels"`
			Notes            []interface{} `json:"notes"`
			RelatedWords     []interface{} `json:"relatedWords"`
			TextProns        []interface{} `json:"textProns"`
		} `json:"items"`
	}{}
	err = json.NewDecoder(res.Body).Decode(&data)
	if err != nil {
		return "", err
	}
	if len(data.WordnikDefinition) == 0 {
		return "", errors.New("no definitions found")
	}

	return data.WordnikDefinition[0].Text, nil
}
