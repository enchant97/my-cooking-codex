package routes

import (
	"net/http"

	"github.com/enchant97/my-cooking-codex/api/core"
	"github.com/enchant97/my-cooking-codex/api/db/crud"
	"github.com/labstack/echo/v4"
)

type accountStats struct {
	UserCount   int64 `json:"userCount"`
	RecipeCount int64 `json:"recipeCount"`
}

func getAccountStats(ctx echo.Context) error {
	authenticatedUser, _ := core.GetAuthenticatedUserFromContext(ctx)

	recipeCount, err := crud.GetRecipesByUserIDCount(authenticatedUser.UserID)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}
	userCount, err := crud.GetUserCount()
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}
	return ctx.JSON(http.StatusOK, accountStats{
		UserCount:   userCount,
		RecipeCount: recipeCount,
	})
}
