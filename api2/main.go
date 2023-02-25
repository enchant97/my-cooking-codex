package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"path"

	"github.com/enchant97/my-cooking-codex/api2/config"
	"github.com/enchant97/my-cooking-codex/api2/db"
	"github.com/enchant97/my-cooking-codex/api2/routes"
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
	// Parse config
	var appConfig config.AppConfig
	if err := appConfig.ParseConfig(); err != nil {
		log.Fatalln(err)
	}
	// Create data directory if it doesn't exist
	if err := os.MkdirAll(appConfig.DataPath, os.ModePerm); err != nil {
		log.Fatalln(err)
	}
	os.Mkdir(path.Join(appConfig.DataPath, "recipe_images"), os.ModePerm)
	// Connect to database
	if err := db.InitDB(appConfig.DB); err != nil {
		log.Fatalln(err)
	}
	// Create & setup server
	e := echo.New()
	e.Use(middleware.Recover())
	e.Use(middleware.Logger())
	e.Use(middleware.CORS())
	e.Validator = &Validator{validator: validator.New()}
	e.Use(func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(ctx echo.Context) error {
			ctx.Set("AppConfig", appConfig)
			return next(ctx)
		}
	})
	routes.InitRoutes(e, appConfig)
	// Start server
	e.Logger.Fatal(e.Start(fmt.Sprintf("%s:%d", appConfig.Host, appConfig.Port)))
}
