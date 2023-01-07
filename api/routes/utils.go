package routes

import (
	"github.com/labstack/echo/v4"
)

func InitRoutes(e *echo.Echo) {
	e.GET("/", func(ctx echo.Context) error {
		return ctx.HTML(200, "<h1>API Backend Operational</h1>")
	})

	usersGroup := e.Group("/api/users")
	{
		usersGroup.POST("/", postCreateUser)
	}
}
