package routes

import (
	"github.com/enchant97/my-cooking-codex/api/config"
	"github.com/enchant97/my-cooking-codex/api/core"
	"github.com/golang-jwt/jwt/v4"
	echojwt "github.com/labstack/echo-jwt/v4"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
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

	apiRoutes := e.Group("/api/", jwtMiddleware)
	{
		apiRoutes.GET("users/me/", getUserMe)
		apiRoutes.POST("recipes/", postCreateRecipe)
		apiRoutes.GET("recipes/", getRecipes)
		apiRoutes.GET("recipes/:id/", getRecipe)
		apiRoutes.PATCH("recipes/:id/", patchRecipe)
		apiRoutes.POST("recipes/:id/image/", postSetRecipeImage, middleware.BodyLimit("4M"))
		apiRoutes.DELETE("recipes/:id/image/", deleteRecipeImage)
		apiRoutes.GET("stats/me/", getAccountStats)
	}

	mediaRoutes := e.Group("/media/")
	{
		mediaRoutes.GET("recipe-image/:id", getRecipeImageContent)
	}
}
