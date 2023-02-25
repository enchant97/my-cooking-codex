package db

import "github.com/google/uuid"

type RecipeIngredient struct {
	Name        string  `json:"name" validate:"required"`
	Amount      float32 `json:"amount" validate:"required"`
	UnitType    string  `json:"unitType" validate:"required"`
	Description *string `json:"description,omitempty"`
}

type RecipeStep struct {
	Title       *string `json:"title,omitempty"`
	Description string  `json:"description" validate:"required"`
}

type CreateUser struct {
	Username string `json:"username" validate:"required"`
	Password string `json:"password" validate:"required"`
}

func (u *CreateUser) IntoUser() User {
	user := User{
		Username: u.Username,
	}
	user.SetPassword(u.Password)
	return user
}

type CreateRecipe struct {
	Title            string             `json:"title" validate:"required"`
	ShortDescription *string            `json:"shortDescription,omitempty"`
	LongDescription  *string            `json:"longDescription,omitempty"`
	Tags             []string           `json:"tags,omitempty"`
	Ingredients      []RecipeIngredient `json:"ingredients,omitempty"`
	Steps            []RecipeStep       `json:"steps,omitempty"`
}

func (r *CreateRecipe) IntoRecipe(ownerID uuid.UUID, hasImage bool) Recipe {
	return Recipe{
		OwnerID:          ownerID,
		Title:            r.Title,
		ShortDescription: r.ShortDescription,
		LongDescription:  r.LongDescription,
		HasImage:         hasImage,
	}
}

type UpdateIngredient struct {
	Name        *string  `json:"name,omitempty"`
	Amount      *float32 `json:"amount,omitempty"`
	UnitType    *string  `json:"unitType,omitempty"`
	Description *string  `json:"description,omitempty"`
}

type UpdateStep struct {
	Title       *string `json:"title,omitempty"`
	Description *string `json:"description,omitempty"`
}

type UpdateRecipe struct {
	Title            *string             `json:"title,omitempty"`
	ShortDescription *string             `json:"shortDescription,omitempty"`
	LongDescription  *string             `json:"longDescription,omitempty"`
	Tags             *[]string           `json:"tags,omitempty"`
	Ingredients      *[]UpdateIngredient `json:"ingredients,omitempty"`
	Steps            *[]UpdateStep       `json:"steps,omitempty"`
	HasImage         *bool               `json:"hasImage,omitempty"`
}
