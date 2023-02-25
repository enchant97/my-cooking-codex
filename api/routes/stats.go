package routes

import (
	"net/http"

	"github.com/enchant97/my-cooking-codex/api/core"
	"github.com/enchant97/my-cooking-codex/api/db"
	"github.com/labstack/echo/v4"
)

type accountStats struct {
	UserCount   int `json:"userCount"`
	RecipeCount int `json:"recipeCount"`
}

func getAccountStats(ctx echo.Context) error {
	username, _ := core.GetAuthenticatedUserFromContext(ctx)

	recipeCount, err := db.GetRecipesByUsernameCount(username)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	userCount, err := db.GetUserCount()
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	return ctx.JSON(http.StatusOK, accountStats{
		UserCount:   userCount,
		RecipeCount: recipeCount,
	})
}
