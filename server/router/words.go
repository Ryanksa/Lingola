package router

import (
	"strings"

	"lingola-server/models"
	"lingola-server/utils"

	"github.com/gin-gonic/gin"
)

type Word struct {
	Word       string `json:"word"`
	Definition string `json:"definition"`
}

type PostWordBody struct {
	Word       string `json:"word"`
	Definition string `json:"definition"`
}

type DeleteWordBody struct {
	Word string `json:"word"`
}

func getWord(c *gin.Context) {
	word := c.Param("word")
	if word == "" {
		c.JSON(400, gin.H{
			"message": "Word required",
		})
		return
	}

	wordObj, err := models.GetWord(word)
	if err != nil {
		c.JSON(500, gin.H{
			"message": "Internal server error",
		})
		return
	}

	c.JSON(200, gin.H{
		"word":       wordObj.Word,
		"definition": wordObj.Definition,
	})
}

func getRandomWords(c *gin.Context) {
	words, err := utils.GetRandomWords(2)
	if err != nil || len(words) != 2 {
		c.JSON(500, gin.H{
			"message": "Internal server error",
		})
		return
	}

	channels := [2]chan string{}
	for i, word := range words {
		channel := make(chan string)
		channels[i] = channel
		go utils.GetWordDefinition(word, channel)
	}

	wordsAndDefs := [2]Word{}
	for i := 0; i < 2; i++ {
		select {
		case def := <-channels[0]:
			wordsAndDefs[0] = Word{
				Word:       words[0],
				Definition: def,
			}
		case def := <-channels[1]:
			wordsAndDefs[1] = Word{
				Word:       words[1],
				Definition: def,
			}
		}
	}

	c.JSON(200, wordsAndDefs)
}

func postWord(c *gin.Context) {
	var body PostWordBody
	if err := c.BindJSON(&body); err != nil {
		c.JSON(400, gin.H{
			"message": "Word and definition required",
		})
		return
	}
	body.Word = strings.ToLower(body.Word)

	wordObj, err := models.CreateWord(body.Word, body.Definition)
	if err != nil {
		c.JSON(500, gin.H{
			"message": "Internal server error",
		})
		return
	}

	c.JSON(200, gin.H{
		"word":       wordObj.Word,
		"definition": wordObj.Definition,
	})
}

func deleteWord(c *gin.Context) {
	var body DeleteWordBody
	if err := c.BindJSON(&body); err != nil {
		c.JSON(400, gin.H{
			"message": "Word required",
		})
		return
	}

	err := models.DeleteWord(body.Word)
	if err != nil {
		c.JSON(500, gin.H{
			"message": "Internal server error",
		})
		return
	}

	c.JSON(200, gin.H{
		"word": body.Word,
	})
}
