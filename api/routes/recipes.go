package routes

import (
	"net/http"

	"github.com/enchant97/recipes/api/core"
	"github.com/enchant97/recipes/api/db"
	"github.com/golang-jwt/jwt/v4"
	"github.com/labstack/echo/v4"
)

func postCreateRecipe(ctx echo.Context) error {
	var recipeData core.CreateRecipe
	if err := ctx.Bind(&recipeData); err != nil {
		return ctx.NoContent(http.StatusBadRequest)
	}
	if err := ctx.Validate(recipeData); err != nil {
		return err
	}
	userToken := ctx.Get("user").(*jwt.Token)
	tokenClaims := userToken.Claims.(*core.JWTClaims)
	username := tokenClaims.Username
	recipe, err := db.CreateRecipe(recipeData.IntoRecipe(username))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	return ctx.JSON(http.StatusCreated, recipe)
}

func getRecipes(ctx echo.Context) error {
	userToken := ctx.Get("user").(*jwt.Token)
	tokenClaims := userToken.Claims.(*core.JWTClaims)
	username := tokenClaims.Username
	recipes, err := db.GetRecipesByUsername(username)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	return ctx.JSON(http.StatusOK, recipes)
}

func getRecipe(ctx echo.Context) error {
	recipeID := ctx.Param("id")
	userToken := ctx.Get("user").(*jwt.Token)
	tokenClaims := userToken.Claims.(*core.JWTClaims)
	username := tokenClaims.Username
	recipe, err := db.GetRecipeById(recipeID, username)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	return ctx.JSON(http.StatusOK, recipe)
}
