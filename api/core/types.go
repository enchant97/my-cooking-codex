package core

import (
	"github.com/enchant97/recipes/api/db"
	"github.com/golang-jwt/jwt/v4"
)

type JWTClaims struct {
	Username string `json:"username"`
	IsAdmin  bool   `json:"isAdmin"`
	jwt.RegisteredClaims
}

type CreateUser struct {
	Username string `json:"username" validate:"required"`
	Password string `json:"password" validate:"required"`
}

func (u *CreateUser) IntoUser() db.User {
	user := db.User{
		Username: u.Username,
	}
	user.SetPassword(u.Password)
	return user
}

type CreateIngredient struct {
	Name        string  `json:"name" validate:"required"`
	Amount      uint    `json:"amount" validate:"required"`
	UnitType    string  `json:"unitType" validate:"required"`
	Description *string `json:"description,omitempty"`
}

func (i *CreateIngredient) IntoIngredient() db.Ingredient {
	return db.Ingredient{
		Name:        i.Name,
		Amount:      i.Amount,
		UnitType:    i.UnitType,
		Description: i.Description,
	}
}

type CreateStep struct {
	Title       *string `json:"title,omitempty"`
	Description string  `json:"description" validate:"required"`
}

func (s *CreateStep) IntoStep() db.Step {
	return db.Step{
		Title:       s.Title,
		Description: s.Description,
	}
}

type CreateRecipe struct {
	Title            string             `json:"title" validate:"required"`
	ShortDescription *string            `json:"shortDescription,omitempty"`
	LongDescription  *string            `json:"longDescription,omitempty"`
	Tags             []string           `json:"tags,omitempty"`
	Ingredients      []CreateIngredient `json:"ingredients,omitempty"`
	Steps            []CreateStep       `json:"steps,omitempty"`
}

func (r *CreateRecipe) IntoRecipe(ownerID string) db.Recipe {
	return db.Recipe{
		OwnerID:          ownerID,
		Title:            r.Title,
		ShortDescription: r.ShortDescription,
		LongDescription:  r.LongDescription,
		Tags:             r.Tags,
		Ingredients: func() []db.Ingredient {
			ingredients := make([]db.Ingredient, len(r.Ingredients))
			for i, v := range r.Ingredients {
				ingredients[i] = v.IntoIngredient()
			}
			return ingredients
		}(),
		Steps: func() []db.Step {
			steps := make([]db.Step, len(r.Steps))
			for i, v := range r.Steps {
				steps[i] = v.IntoStep()
			}
			return steps
		}(),
	}
}
