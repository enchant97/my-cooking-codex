package db

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
