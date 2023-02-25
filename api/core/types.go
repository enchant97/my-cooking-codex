package core

import (
	"time"

	"github.com/golang-jwt/jwt/v4"
)

type JWTClaims struct {
	Username string `json:"username"`
	IsAdmin  bool   `json:"isAdmin"`
	jwt.RegisteredClaims
}

type LoginToken struct {
	Type   string    `json:"type"`
	Token  string    `json:"token"`
	Expiry time.Time `json:"expiry"`
}

type CreateLogin struct {
	Username string `json:"username" validate:"required"`
	Password string `json:"password" validate:"required"`
}
