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
	"github.com/labstack/echo/v4/middleware"
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
	e.Use(middleware.Recover())
	e.Use(middleware.Logger())
	e.Validator = &Validator{validator: validator.New()}
	e.Use(func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(ctx echo.Context) error {
			ctx.Set("AppConfig", appConfig)
			return next(ctx)
		}
	})
	routes.InitRoutes(e, appConfig)

	e.Logger.Fatal(e.Start(fmt.Sprintf("%s:%d", appConfig.Host, appConfig.Port)))
}
