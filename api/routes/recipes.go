package routes

import (
	"bytes"
	"io"
	"net/http"
	"strings"

	"github.com/enchant97/my-cooking-codex/api/core"
	"github.com/enchant97/my-cooking-codex/api/db"
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

func patchRecipe(ctx echo.Context) error {
	recipeID := ctx.Param("id")
	userToken := ctx.Get("user").(*jwt.Token)
	tokenClaims := userToken.Claims.(*core.JWTClaims)
	username := tokenClaims.Username

	// validate whether user can modify the recipe content
	isOwner, err := db.DoesUserOwnRecipe(username, recipeID)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	} else if !isOwner {
		return ctx.NoContent(http.StatusForbidden)
	}

	var recipeData core.UpdateRecipe
	if err := ctx.Bind(&recipeData); err != nil {
		return ctx.NoContent(http.StatusBadRequest)
	}
	if err := ctx.Validate(recipeData); err != nil {
		return err
	}

	if db.UpdateRecipe(recipeID, recipeData) != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}

	return ctx.NoContent(http.StatusNoContent)
}

func postSetRecipeImage(ctx echo.Context) error {
	recipeID := ctx.Param("id")
	userToken := ctx.Get("user").(*jwt.Token)
	tokenClaims := userToken.Claims.(*core.JWTClaims)
	username := tokenClaims.Username

	// TODO validate Content-Type & extract with error handling
	imageType := strings.Split(ctx.Request().Header.Get("Content-Type"), "/")[1]

	// validate whether user can modify the recipe content
	isOwner, err := db.DoesUserOwnRecipe(username, recipeID)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	} else if !isOwner {
		return ctx.NoContent(http.StatusForbidden)
	}

	var content = make([]byte, ctx.Request().ContentLength)
	// TODO handle errors
	var b = bytes.Buffer{}
	io.Copy(&b, ctx.Request().Body)
	b.Read(content)

	recipeImageToCreate := core.CreateRecipeImage{
		RecipeID:  recipeID,
		ImageType: imageType,
	}
	recipeImage := recipeImageToCreate.IntoRecipeImage(content)
	recipeImage, err = db.SetRecipeImage(recipeImage)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	// HACK make specific method for this
	if err := db.UpdateRecipe(recipeID, map[string]interface{}{"hasImage": true}); err != nil {
		ctx.Logger().Error(err)
	}

	return ctx.JSON(http.StatusCreated, recipeImage)
}

func deleteRecipeImage(ctx echo.Context) error {
	recipeID := ctx.Param("id")
	userToken := ctx.Get("user").(*jwt.Token)
	tokenClaims := userToken.Claims.(*core.JWTClaims)
	username := tokenClaims.Username

	// validate whether user can modify the recipe content
	isOwner, err := db.DoesUserOwnRecipe(username, recipeID)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	} else if !isOwner {
		return ctx.NoContent(http.StatusForbidden)
	}

	if err := db.DeleteRecipeImage(recipeID); err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	// HACK make specific method for this
	if err := db.UpdateRecipe(recipeID, map[string]interface{}{"hasImage": false}); err != nil {
		ctx.Logger().Error(err)
	}

	return ctx.NoContent(http.StatusNoContent)
}
