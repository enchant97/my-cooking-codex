package routes

import (
	"bytes"
	"io"
	"net/http"
	"os"
	"path"

	"github.com/enchant97/my-cooking-codex/api/config"
	"github.com/enchant97/my-cooking-codex/api/core"
	"github.com/enchant97/my-cooking-codex/api/db"
	"github.com/enchant97/my-cooking-codex/api/db/crud"
	"github.com/google/uuid"
	"github.com/labstack/echo/v4"
)

func postCreateRecipe(ctx echo.Context) error {
	username, _ := core.GetAuthenticatedUserFromContext(ctx)
	userID, _ := crud.GetUserIDByUsername(username)

	var recipeData db.CreateRecipe
	if err := ctx.Bind(&recipeData); err != nil {
		return ctx.NoContent(http.StatusBadRequest)
	}
	if err := ctx.Validate(recipeData); err != nil {
		return err
	}

	recipe, err := crud.CreateRecipe(recipeData.IntoRecipe(userID, false))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}
	return ctx.JSON(http.StatusCreated, recipe)
}

func getRecipes(ctx echo.Context) error {
	username, _ := core.GetAuthenticatedUserFromContext(ctx)
	userID, _ := crud.GetUserIDByUsername(username)

	recipes, err := crud.GetRecipesByUserID(userID)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}
	return ctx.JSON(http.StatusOK, recipes)
}

func getRecipe(ctx echo.Context) error {
	recipeID := ctx.Param("id")
	username, _ := core.GetAuthenticatedUserFromContext(ctx)
	userID, _ := crud.GetUserIDByUsername(username)

	if hasAccess, err := crud.DoesUserOwnRecipe(userID, uuid.MustParse(recipeID)); err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	} else if !hasAccess {
		return ctx.NoContent(http.StatusNotFound)
	}

	recipe, err := crud.GetRecipeById(uuid.MustParse(recipeID))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}
	return ctx.JSON(http.StatusOK, recipe)
}

func patchRecipe(ctx echo.Context) error {
	recipeID := ctx.Param("id")
	username, _ := core.GetAuthenticatedUserFromContext(ctx)
	userID, _ := crud.GetUserIDByUsername(username)

	// validate whether user can modify the recipe content
	isOwner, err := crud.DoesUserOwnRecipe(userID, uuid.MustParse(recipeID))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	} else if !isOwner {
		return ctx.NoContent(http.StatusForbidden)
	}

	var recipeData db.UpdateRecipe
	if err := ctx.Bind(&recipeData); err != nil {
		return ctx.NoContent(http.StatusBadRequest)
	}
	if err := ctx.Validate(recipeData); err != nil {
		return err
	}

	if _, err := crud.UpdateRecipe(uuid.MustParse(recipeID), recipeData); err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}

	return ctx.NoContent(http.StatusNoContent)
}

func deleteRecipe(ctx echo.Context) error {
	appConfig := ctx.Get("AppConfig").(config.AppConfig)
	recipeID := ctx.Param("id")
	username, _ := core.GetAuthenticatedUserFromContext(ctx)
	userID, _ := crud.GetUserIDByUsername(username)

	// validate whether user can modify the recipe content
	isOwner, err := crud.DoesUserOwnRecipe(userID, uuid.MustParse(recipeID))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	} else if !isOwner {
		return ctx.NoContent(http.StatusForbidden)
	}

	if err := crud.DeleteRecipe(uuid.MustParse(recipeID)); err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}

	os.Remove(path.Join(appConfig.DataPath, "recipe_images", uuid.MustParse(recipeID).String()+".jpg"))

	return ctx.NoContent(http.StatusNoContent)
}

func postSetRecipeImage(ctx echo.Context) error {
	appConfig := ctx.Get("AppConfig").(config.AppConfig)
	recipeID := ctx.Param("id")
	username, _ := core.GetAuthenticatedUserFromContext(ctx)
	userID, _ := crud.GetUserIDByUsername(username)

	// validate whether user can modify the recipe content
	isOwner, err := crud.DoesUserOwnRecipe(userID, uuid.MustParse(recipeID))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
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
		return ctx.NoContent(http.StatusInternalServerError)
	}

	if err := os.WriteFile(path.Join(appConfig.DataPath, "recipe_images", uuid.MustParse(recipeID).String()+".jpg"), content, 0644); err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}

	if err := crud.UpdateRecipeHasImage(uuid.MustParse(recipeID), true); err != nil {
		ctx.Logger().Error(err)
	}

	return ctx.NoContent(http.StatusNoContent)
}

func deleteRecipeImage(ctx echo.Context) error {
	appConfig := ctx.Get("AppConfig").(config.AppConfig)
	recipeID := ctx.Param("id")
	username, _ := core.GetAuthenticatedUserFromContext(ctx)
	userID, _ := crud.GetUserIDByUsername(username)

	// validate whether user can modify the recipe content
	isOwner, err := crud.DoesUserOwnRecipe(userID, uuid.MustParse(recipeID))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	} else if !isOwner {
		return ctx.NoContent(http.StatusForbidden)
	}

	os.Remove(path.Join(appConfig.DataPath, "recipe_images", uuid.MustParse(recipeID).String()+".jpg"))

	if err := crud.UpdateRecipeHasImage(uuid.MustParse(recipeID), false); err != nil {
		ctx.Logger().Error(err)
	}

	return ctx.NoContent(http.StatusNoContent)
}
