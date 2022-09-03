package router

import (
	"os"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

func GetRouter() *gin.Engine {
	host := os.Getenv("HOST")
	cors_origin := os.Getenv("CORS_ORIGIN")

	router := gin.Default()
	router.SetTrustedProxies([]string{host})
	router.Use(cors.New(cors.Config{
		AllowOrigins: []string{cors_origin},
	}))

	wordGroup := router.Group("/api/word")
	{
		wordGroup.GET("/one/:word", getWord)
		wordGroup.GET("/random", getRandomWords)
		wordGroup.POST("/create", postWord)
		wordGroup.POST("/delete", deleteWord)
	}

	return router
}
