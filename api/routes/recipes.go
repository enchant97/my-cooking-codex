package routes

import (
	"bytes"
	"io"
	"net/http"

	"github.com/enchant97/my-cooking-codex/api/core"
	"github.com/enchant97/my-cooking-codex/api/db"
	"github.com/labstack/echo/v4"
)

func postCreateRecipe(ctx echo.Context) error {
	username, _ := core.GetAuthenticatedUserFromContext(ctx)

	var recipeData core.CreateRecipe
	if err := ctx.Bind(&recipeData); err != nil {
		return ctx.NoContent(http.StatusBadRequest)
	}
	if err := ctx.Validate(recipeData); err != nil {
		return err
	}

	recipe, err := db.CreateRecipe(recipeData.IntoRecipe(username))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	return ctx.JSON(http.StatusCreated, recipe)
}

func getRecipes(ctx echo.Context) error {
	username, _ := core.GetAuthenticatedUserFromContext(ctx)

	recipes, err := db.GetRecipesByUsername(username)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	return ctx.JSON(http.StatusOK, recipes)
}

func getRecipe(ctx echo.Context) error {
	recipeID := ctx.Param("id")
	username, _ := core.GetAuthenticatedUserFromContext(ctx)

	recipe, err := db.GetRecipeById(recipeID, username)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}
	return ctx.JSON(http.StatusOK, recipe)
}

func patchRecipe(ctx echo.Context) error {
	recipeID := ctx.Param("id")
	username, _ := core.GetAuthenticatedUserFromContext(ctx)

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

func deleteRecipe(ctx echo.Context) error {
	recipeID := ctx.Param("id")
	username, _ := core.GetAuthenticatedUserFromContext(ctx)

	// validate whether user can modify the recipe content
	isOwner, err := db.DoesUserOwnRecipe(username, recipeID)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	} else if !isOwner {
		return ctx.NoContent(http.StatusForbidden)
	}

	if err := db.DeleteRecipe(recipeID); err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}

	if err := db.DeleteRecipeImage(recipeID); err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}

	return ctx.NoContent(http.StatusNoContent)
}

func postSetRecipeImage(ctx echo.Context) error {
	recipeID := ctx.Param("id")
	username, _ := core.GetAuthenticatedUserFromContext(ctx)

	// TODO validate Content-Type & extract with error handling
	//imageType := strings.Split(ctx.Request().Header.Get("Content-Type"), "/")[1]

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
	if optimisedContent, err := core.OptimiseImageToJPEG(content, 2000); err == nil {
		content = optimisedContent
	} else {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}

	recipeImageToCreate := core.CreateRecipeImage{
		RecipeID:  recipeID,
		ImageType: "jpeg",
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
	username, _ := core.GetAuthenticatedUserFromContext(ctx)

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
