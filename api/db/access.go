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

func GetUserByUsername(username string) (User, error) {
	cursor, err := r.Table(TableNameUsers).Get(username).Run(session)
	if err != nil {
		return User{}, err
	}
	defer cursor.Close()
	var user User
	err = cursor.One(&user)
	return user, err
}
