package core

import "github.com/golang-jwt/jwt/v4"

type JWTClaims struct {
	Username string `json:"username"`
	IsAdmin  bool   `json:"isAdmin"`
	jwt.RegisteredClaims
}

type CreateUser struct {
	Username string `json:"username" validate:"required"`
	Password string `json:"password" validate:"required"`
}

type CreateIngredient struct {
	Name        string  `json:"name" validate:"required"`
	Amount      uint    `json:"amount" validate:"required"`
	UnitType    string  `json:"unitType" validate:"required"`
	Description *string `json:"description,omitempty"`
}

type CreateStep struct {
	Title       *string `json:"title,omitempty"`
	Description string  `json:"description" validate:"required"`
}

type CreateRecipe struct {
	Title            string             `json:"title" validate:"required"`
	ShortDescription *string            `json:"shortDescription,omitempty"`
	LongDescription  *string            `json:"longDescription,omitempty"`
	Tags             []string           `json:"tags,omitempty"`
	Ingredients      []CreateIngredient `json:"ingredients,omitempty"`
	Steps            []CreateStep       `json:"steps,omitempty"`
}
