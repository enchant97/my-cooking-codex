package core

import (
	"time"

	"github.com/golang-jwt/jwt/v4"
	"github.com/labstack/echo/v4"
)

// Get the authenticated user from the context
// and return the username and the JWTClaims
func GetAuthenticatedUserFromContext(ctx echo.Context) (string, JWTClaims) {
	userToken := ctx.Get("user").(*jwt.Token)
	tokenClaims := userToken.Claims.(*JWTClaims)
	return tokenClaims.Username, *tokenClaims
}

// Create token for authentication
func CreateAuthenticationToken(username string, isAdmin bool, secretKey []byte) (LoginToken, error) {
	expiresAt := time.Now().Add(time.Hour * 72)
	claims := &JWTClaims{
		Username: username,
		IsAdmin:  isAdmin,
		RegisteredClaims: jwt.RegisteredClaims{
			ExpiresAt: jwt.NewNumericDate(expiresAt),
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	rawToken, err := token.SignedString(secretKey)
	if err != nil {
		return LoginToken{}, err
	}
	return LoginToken{
		Type:   "Bearer",
		Token:  rawToken,
		Expiry: expiresAt,
	}, nil
}
