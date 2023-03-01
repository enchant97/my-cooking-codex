package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"path"

	"github.com/enchant97/my-cooking-codex/api/config"
	"github.com/enchant97/my-cooking-codex/api/core"
	"github.com/enchant97/my-cooking-codex/api/db"
	"github.com/enchant97/my-cooking-codex/api/routes"
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
	os.MkdirAll(path.Join(appConfig.DataPath, core.RecipeImagesOriginalPath), os.ModePerm)
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
	if appConfig.StaticPath != nil {
		log.Println("Serving static files from", *appConfig.StaticPath)
		e.Use(middleware.StaticWithConfig(middleware.StaticConfig{
			Root:  *appConfig.StaticPath,
			HTML5: true,
		}))
	} else {
		e.GET("/", func(ctx echo.Context) error {
			return ctx.HTML(200, "<h1>API Backend Operational</h1>")
		})
	}
	// Start server
	e.Logger.Fatal(e.Start(fmt.Sprintf("%s:%d", appConfig.Host, appConfig.Port)))
}
