package router

import (
	"os"

	"github.com/gin-gonic/gin"
)

func GetRouter() *gin.Engine {
	host := os.Getenv("HOST")

	router := gin.Default()
	router.SetTrustedProxies([]string{host})

	wordGroup := router.Group("/word")
	{
		wordGroup.GET("/one/:word", getWord)
		wordGroup.GET("/random", getRandomWords)
		wordGroup.POST("/create", postWord)
		wordGroup.POST("/delete", deleteWord)
	}

	return router
}
