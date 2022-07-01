package main

import (
	"fmt"
	"os"

	"lingola-server/models"
	"lingola-server/router"
	"lingola-server/utils"
)

func main() {
	utils.LoadEnv()

	port := os.Getenv("PORT")

	models.SetupDB()

	router := router.GetRouter()
	router.Run(fmt.Sprintf(":%s", port))
}
