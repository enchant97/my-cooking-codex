package main

import (
	"log"

	"github.com/enchant97/recipes/api/config"
	"github.com/enchant97/recipes/api/db"
)

func main() {
	var appConfig config.AppConfig
	if err := appConfig.ParseConfig(); err != nil {
		log.Fatalln(err)
	}
	if err := db.InitDB(appConfig.DB); err != nil {
		log.Fatalln(err)
	}

	defer db.CloseDB()
}
