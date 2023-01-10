package routes

import (
	"net/http"
	"time"

	"github.com/enchant97/recipes/api/config"
	"github.com/enchant97/recipes/api/core"
	"github.com/enchant97/recipes/api/db"
	"github.com/golang-jwt/jwt/v4"
	"github.com/labstack/echo/v4"
)

type login struct {
	Username string `json:"username" validate:"required"`
	Password string `json:"password" validate:"required"`
}

type loginToken struct {
	Type  string `json:"type"`
	Token string `json:"token"`
}

func postLogin(ctx echo.Context) error {
	appConfig := ctx.Get("AppConfig").(config.AppConfig)
	var loginData login
	if err := ctx.Bind(&loginData); err != nil {
		return ctx.NoContent(http.StatusBadRequest)
	}
	if err := ctx.Validate(loginData); err != nil {
		return err
	}

	// validate username & password
	user, err := db.GetUserByUsername(loginData.Username)
	// TODO catch the specific db error
	if err != nil || !user.IsPasswordMatch(loginData.Password) {
		return ctx.NoContent(http.StatusUnauthorized)
	}

	claims := &core.JWTClaims{
		Username: loginData.Username,
		IsAdmin:  false,
		RegisteredClaims: jwt.RegisteredClaims{
			ExpiresAt: jwt.NewNumericDate(time.Now().Add(time.Hour * 72)),
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	rawToken, err := token.SignedString([]byte(appConfig.SecretKey))
	if err != nil {
		return err
	}

	return ctx.JSON(200, loginToken{
		Type:  "Bearer",
		Token: rawToken,
	})
}
