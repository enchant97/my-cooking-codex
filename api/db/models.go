package db

import (
	"golang.org/x/crypto/bcrypt"
)

type User struct {
	ID             string `rethinkdb:"id,omitempty" json:"id"`
	Username       string `rethinkdb:"username" json:"username"`
	HashedPassword []byte `rethinkdb:"hashedPassword" json:"-"`
}

// Set a new password (hashing it)
func (u *User) SetPassword(newPlainPassword string) error {
	hashedPw, err := bcrypt.GenerateFromPassword([]byte(newPlainPassword), bcrypt.DefaultCost)
	if err != nil {
		return err
	}
	u.HashedPassword = hashedPw
	return nil
}

// Check if password matches the hashed stored one
func (u *User) IsPasswordMatch(plainPassword string) bool {
	if err := bcrypt.CompareHashAndPassword(u.HashedPassword, []byte(plainPassword)); err == nil {
		return true
	}
	return false
}

type Ingredient struct {
	Name     string `rethinkdb:"name" json:"name"`
	Amount   int    `rethinkdb:"amount" json:"amount"`
	UnitType string `rethinkdb:"unitType" json:"unitType"`
}

type Recipe struct {
	ID               string       `rethinkdb:"id,omitempty" json:"id"`
	OwnerID          string       `rethinkdb:"ownerId" json:"ownerId"`
	Title            string       `rethinkdb:"title" json:"title"`
	ShortDescription *string      `rethinkdb:"shortDescription,omitempty" json:"shortDescription,omitempty"`
	LongDescription  *string      `rethinkdb:"longDescription,omitempty" json:"longDescription,omitempty"`
	ThumbnailName    *[]string    `rethinkdb:"thumbnailName,omitempty" json:"thumbnailName,omitempty"`
	Tags             []string     `rethinkdb:"tags,omitempty" json:"tags,omitempty"`
	Ingredients      []Ingredient `rethinkdb:"ingredients,omitempty" json:"ingredients,omitempty"`
	Steps            []string     `rethinkdb:"steps,omitempty" json:"steps,omitempty"`
}
