package routes

import (
	"net/http"

	"github.com/enchant97/recipes/api/core"
	"github.com/enchant97/recipes/api/db"
	"github.com/golang-jwt/jwt/v4"
	"github.com/labstack/echo/v4"
)

func postCreateUser(ctx echo.Context) error {
	var userData core.CreateUser
	if err := ctx.Bind(&userData); err != nil {
		return ctx.NoContent(http.StatusBadRequest)
	}
	if err := ctx.Validate(userData); err != nil {
		return err
	}
	user, err := db.CreateUser(userData)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(500)
	}

	return ctx.JSON(201, user)
}

func getUserMe(ctx echo.Context) error {
	userToken := ctx.Get("user").(*jwt.Token)
	tokenClaims := userToken.Claims.(*core.JWTClaims)

	user, err := db.GetUserByUsername(tokenClaims.Username)
	if err != nil {
		ctx.Logger().Error(err)
		return ctx.NoContent(http.StatusInternalServerError)
	}

	return ctx.JSON(http.StatusOK, user)
}
