package routes

import (
	"fmt"
	"net/http"

	"github.com/enchant97/recipes/api/db"
	"github.com/labstack/echo/v4"
)

func getRecipeImageContent(ctx echo.Context) error {
	imageID := ctx.Param("id")
	recipeImage, err := db.GetRecipeImage(imageID)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	ctx.Response().Header().Add("Recipe-Id", recipeImage.RecipeID)
	return ctx.Blob(http.StatusOK, fmt.Sprintf("image/%s", recipeImage.ImageType), recipeImage.Content)
}
