package db

import (
	"github.com/enchant97/recipes/api/core"
	r "gopkg.in/rethinkdb/rethinkdb-go.v6"
)

func CreateUser(userData core.CreateUser) (User, error) {
	user := User{
		Username: userData.Username,
	}
	user.SetPassword(userData.Password)
	if _, err := r.Table(TableNameUsers).Insert(&user).RunWrite(session); err != nil {
		return User{}, err
	}
	return user, nil
}
