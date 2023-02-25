package routes

import (
	"net/http"

	"github.com/enchant97/my-cooking-codex/api2/config"
	"github.com/enchant97/my-cooking-codex/api2/core"
	"github.com/enchant97/my-cooking-codex/api2/db/crud"
	"github.com/labstack/echo/v4"
)

func postLogin(ctx echo.Context) error {
	appConfig := ctx.Get("AppConfig").(config.AppConfig)
	var loginData core.CreateLogin
	if err := ctx.Bind(&loginData); err != nil {
		return ctx.NoContent(http.StatusBadRequest)
	}
	if err := ctx.Validate(loginData); err != nil {
		return err
	}

	// validate username & password
	user, err := crud.GetUserByUsername(loginData.Username)
	// TODO catch the specific db error
	if err != nil || !user.IsPasswordMatch(loginData.Password) {
		return ctx.NoContent(http.StatusUnauthorized)
	}

	// user is valid, create a token
	if token, err := core.CreateAuthenticationToken(
		loginData.Username,
		false,
		[]byte(appConfig.SecretKey),
	); err != nil {
		return err
	} else {
		return ctx.JSON(http.StatusOK, token)
	}
}
