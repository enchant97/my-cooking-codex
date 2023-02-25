package routes

import (
	"path"

	"github.com/enchant97/my-cooking-codex/api/config"
	"github.com/google/uuid"
	"github.com/labstack/echo/v4"
)

func getRecipeImageContent(ctx echo.Context) error {
	appConfig := ctx.Get("AppConfig").(config.AppConfig)
	imageID := ctx.Param("id")

	return ctx.File(path.Join(appConfig.DataPath, "recipe_images", uuid.MustParse(imageID).String()+".jpg"))
}
