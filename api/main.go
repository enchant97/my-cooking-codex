package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/enchant97/recipes/api/config"
	"github.com/enchant97/recipes/api/db"
	"github.com/enchant97/recipes/api/routes"
	"github.com/go-playground/validator"
	"github.com/labstack/echo/v4"
)

type Validator struct {
	validator *validator.Validate
}

func (cv *Validator) Validate(i interface{}) error {
	if err := cv.validator.Struct(i); err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}
	return nil
}

func main() {
	var appConfig config.AppConfig
	if err := appConfig.ParseConfig(); err != nil {
		log.Fatalln(err)
	}

	if err := db.InitDB(appConfig.DB); err != nil {
		log.Fatalln(err)
	}
	defer db.CloseDB()

	e := echo.New()
	e.Validator = &Validator{validator: validator.New()}
	routes.InitRoutes(e)

	e.Logger.Fatal(e.Start(fmt.Sprintf("%s:%d", appConfig.Host, appConfig.Port)))
}
