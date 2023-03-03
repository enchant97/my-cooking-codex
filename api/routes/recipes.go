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
	authenticatedUser := getAuthenticatedUser(ctx)

	var recipeData db.CreateRecipe
	if err := ctx.Bind(&recipeData); err != nil {
		return ctx.NoContent(http.StatusBadRequest)
	}
	if err := ctx.Validate(recipeData); err != nil {
		return err
	}

	recipe, err := crud.CreateRecipe(recipeData.IntoRecipe(authenticatedUser.UserID, nil))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}
	return ctx.JSON(http.StatusCreated, recipe)
}

func getRecipes(ctx echo.Context) error {
	authenticatedUser := getAuthenticatedUser(ctx)

	recipes, err := crud.GetRecipesByUserID(authenticatedUser.UserID)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}
	return ctx.JSON(http.StatusOK, recipes)
}

func getRecipe(ctx echo.Context) error {
	recipeID := ctx.Param("id")
	authenticatedUser := getAuthenticatedUser(ctx)

	if hasAccess, err := crud.DoesUserOwnRecipe(authenticatedUser.UserID, uuid.MustParse(recipeID)); err != nil {
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
	authenticatedUser := getAuthenticatedUser(ctx)

	// validate whether user can modify the recipe content
	isOwner, err := crud.DoesUserOwnRecipe(authenticatedUser.UserID, uuid.MustParse(recipeID))
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
	authenticatedUser := getAuthenticatedUser(ctx)

	// validate whether user can modify the recipe content
	isOwner, err := crud.DoesUserOwnRecipe(authenticatedUser.UserID, uuid.MustParse(recipeID))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	} else if !isOwner {
		return ctx.NoContent(http.StatusForbidden)
	}

	recipe, err := crud.GetRecipeById(uuid.MustParse(recipeID))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}

	if err := crud.DeleteRecipe(uuid.MustParse(recipeID)); err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}

	os.Remove(path.Join(appConfig.DataPath, core.RecipeImagesOriginalPath, recipe.ImageID.String()+".jpg"))

	return ctx.NoContent(http.StatusNoContent)
}

func postSetRecipeImage(ctx echo.Context) error {
	appConfig := ctx.Get("AppConfig").(config.AppConfig)
	recipeID := ctx.Param("id")
	authenticatedUser := getAuthenticatedUser(ctx)

	// validate whether user can modify the recipe content
	isOwner, err := crud.DoesUserOwnRecipe(authenticatedUser.UserID, uuid.MustParse(recipeID))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	} else if !isOwner {
		return ctx.NoContent(http.StatusForbidden)
	}

	recipe, err := crud.GetRecipeById(uuid.MustParse(recipeID))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
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

	imageID := uuid.New()

	if err := os.WriteFile(path.Join(appConfig.DataPath, core.RecipeImagesOriginalPath, imageID.String()+".jpg"), content, 0644); err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}

	if err := crud.UpdateRecipeImage(uuid.MustParse(recipeID), &imageID); err != nil {
		ctx.Logger().Error(err)
	}

	// Remove old image if one was set
	if recipe.ImageID != nil {
		os.Remove(path.Join(appConfig.DataPath, core.RecipeImagesOriginalPath, recipe.ImageID.String()+".jpg"))
	}

	return ctx.JSON(http.StatusCreated, imageID.String())
}

func deleteRecipeImage(ctx echo.Context) error {
	appConfig := ctx.Get("AppConfig").(config.AppConfig)
	recipeID := ctx.Param("id")
	authenticatedUser := getAuthenticatedUser(ctx)

	// validate whether user can modify the recipe content
	isOwner, err := crud.DoesUserOwnRecipe(authenticatedUser.UserID, uuid.MustParse(recipeID))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	} else if !isOwner {
		return ctx.NoContent(http.StatusForbidden)
	}

	recipe, err := crud.GetRecipeById(uuid.MustParse(recipeID))
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}

	os.Remove(path.Join(appConfig.DataPath, core.RecipeImagesOriginalPath, recipe.ImageID.String()+".jpg"))

	if err := crud.UpdateRecipeImage(uuid.MustParse(recipeID), nil); err != nil {
		ctx.Logger().Error(err)
	}

	return ctx.NoContent(http.StatusNoContent)
}
