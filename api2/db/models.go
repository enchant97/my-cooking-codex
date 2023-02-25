package db

import "golang.org/x/crypto/bcrypt"

type User struct {
	ID             uint     `gorm:"primarykey" json:"id"`
	Username       string   `gorm:"unique;not null;type:varchar(30)" json:"username"`
	HashedPassword []byte   `gorm:"not null" json:"-"`
	Recipes        []Recipe `gorm:"foreignKey:OwnerID" json:"-"`
}

func (u *User) SetPassword(newPlainPassword string) {
	hashedPw, err := bcrypt.GenerateFromPassword([]byte(newPlainPassword), bcrypt.DefaultCost)
	if err != nil {
		panic(err)
	}
	u.HashedPassword = hashedPw
}

func (u *User) IsPasswordMatch(plainPassword string) bool {
	if err := bcrypt.CompareHashAndPassword(u.HashedPassword, []byte(plainPassword)); err == nil {
		return true
	}
	return false
}

type Recipe struct {
	ID               uint               `gorm:"primarykey" json:"id"`
	OwnerID          uint               `gorm:"not null" json:"ownerId"`
	Title            string             `gorm:"not null;type:varchar(30)" json:"title"`
	ShortDescription *string            `gorm:"type:varchar(256)" json:"shortDescription,omitempty"`
	LongDescription  *string            `json:"longDescription,omitempty"`
	HasImage         bool               `gorm:"not null" json:"hasImage"`
	Ingredients      []RecipeIngredient `gorm:"foreignKey:RecipeID" json:"ingredients,omitempty"`
	Steps            []RecipeStep       `gorm:"foreignKey:RecipeID" json:"steps,omitempty"`
}

type RecipeIngredient struct {
	ID          uint    `gorm:"primarykey" json:"id"`
	RecipeID    uint    `gorm:"not null" json:"recipeId"`
	Name        string  `gorm:"not null;type:varchar(128)" json:"name"`
	Amount      float32 `gorm:"not null" json:"amount"`
	UnitType    string  `gorm:"not null;type:varchar(128)" json:"unitType"`
	Description *string `gorm:"type:varchar(255)" json:"description,omitempty"`
}

type RecipeStep struct {
	ID          uint    `gorm:"primarykey" json:"id"`
	RecipeID    uint    `gorm:"not null;index:idx_member" json:"recipeId"`
	Title       *string `gorm:"type:varchar(128)" json:"title,omitempty"`
	Description string  `gorm:"not null;type:varchar(8000)" json:"description"`
}
