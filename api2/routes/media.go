package routes

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
)

func getRecipeImageContent(ctx echo.Context) error {
	// imageID := ctx.Param("id")

	// TODO Get the image from disk
	content := make([]byte, 0)
	return ctx.Blob(http.StatusOK, fmt.Sprintf("image/%s", "jpeg"), content)
}
