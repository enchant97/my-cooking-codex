package db

import (
	"github.com/google/uuid"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/datatypes"
	"gorm.io/gorm"
)

type UUIDBase struct {
	ID uuid.UUID `gorm:"primarykey;type:uuid" json:"id"`
}

func (base *UUIDBase) BeforeCreate(tx *gorm.DB) (err error) {
	base.ID = uuid.New()
	return
}

type User struct {
	UUIDBase
	Username       string   `gorm:"uniqueIndex;not null;type:varchar(30)" json:"username"`
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
	UUIDBase
	OwnerID          uuid.UUID                               `gorm:"not null" json:"ownerId"`
	Title            string                                  `gorm:"not null;type:varchar(30)" json:"title"`
	ShortDescription *string                                 `gorm:"type:varchar(256)" json:"shortDescription,omitempty"`
	LongDescription  *string                                 `json:"longDescription,omitempty"`
	Ingredients      *datatypes.JSONType[[]RecipeIngredient] `json:"ingredients,omitempty"`
	Steps            *datatypes.JSONType[[]RecipeStep]       `json:"steps,omitempty"`
}
