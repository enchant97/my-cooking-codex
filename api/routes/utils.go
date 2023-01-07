package routes

import (
	"github.com/enchant97/recipes/api/config"
	"github.com/enchant97/recipes/api/core"
	"github.com/golang-jwt/jwt/v4"
	echojwt "github.com/labstack/echo-jwt/v4"
	"github.com/labstack/echo/v4"
)

func InitRoutes(e *echo.Echo, appConfig config.AppConfig) {
	e.GET("/", func(ctx echo.Context) error {
		return ctx.HTML(200, "<h1>API Backend Operational</h1>")
	})
	e.POST("/api/users/", postCreateUser)
	e.POST("/api/login/", postLogin)

	config := echojwt.Config{
		NewClaimsFunc: func(c echo.Context) jwt.Claims {
			return new(core.JWTClaims)
		},
		SigningKey: []byte(appConfig.SecretKey),
	}
	jwtMiddleware := echojwt.WithConfig(config)

	apiRoutes := e.Group("/api/")
	{
		apiRoutes.Use(jwtMiddleware)
		apiRoutes.GET("users/me/", getUserMe)
	}

}
