package models

import (
	"gorm.io/gorm"
)

type Word struct {
	gorm.Model
	Word       string `gorm:"index"`
	Definition string
}

func GetWordByID(id uint) (Word, error) {
	var wordObj Word
	result := db.First(&wordObj, id)
	return wordObj, result.Error
}

func GetWord(word string) (Word, error) {
	var wordObj Word
	result := db.Where("word = ?", word).First(&wordObj)
	return wordObj, result.Error
}

func CreateWord(word string, definition string) (Word, error) {
	wordObj := Word{Word: word, Definition: definition}
	result := db.Create(&wordObj)
	return wordObj, result.Error
}

func DeleteWordByID(id uint) error {
	result := db.Delete(&Word{}, id)
	return result.Error
}

func DeleteWord(word string) error {
	result := db.Where("word = ?", word).Delete(&Word{})
	return result.Error
}
