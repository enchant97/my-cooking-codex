package routes

import (
	"net/http"

	"github.com/enchant97/my-cooking-codex/api/core"
	"github.com/enchant97/my-cooking-codex/api/db"
	"github.com/enchant97/my-cooking-codex/api/db/crud"
	"github.com/labstack/echo/v4"
)

func postCreateUser(ctx echo.Context) error {
	var userData db.CreateUser
	if err := ctx.Bind(&userData); err != nil {
		return ctx.NoContent(http.StatusBadRequest)
	}
	if err := ctx.Validate(userData); err != nil {
		return err
	}
	user, err := crud.CreateUser(userData)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}

	return ctx.JSON(http.StatusCreated, user)
}

func getUserMe(ctx echo.Context) error {
	authenticatedUser, _ := core.GetAuthenticatedUserFromContext(ctx)

	user, err := crud.GetRecipeById(authenticatedUser.UserID)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}

	return ctx.JSON(http.StatusOK, user)
}
