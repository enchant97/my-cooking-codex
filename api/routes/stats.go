package routes

import (
	"net/http"

	"github.com/enchant97/my-cooking-codex/api/core"
	"github.com/enchant97/my-cooking-codex/api/db"
	"github.com/golang-jwt/jwt/v4"
	"github.com/labstack/echo/v4"
)

type accountStats struct {
	RecipeCount int `json:"recipeCount"`
}

func getAccountStats(ctx echo.Context) error {
	userToken := ctx.Get("user").(*jwt.Token)
	tokenClaims := userToken.Claims.(*core.JWTClaims)
	username := tokenClaims.Username
	recipeCount, err := db.GetRecipesByUsernameCount(username)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	return ctx.JSON(http.StatusOK, accountStats{
		RecipeCount: recipeCount,
	})
}
