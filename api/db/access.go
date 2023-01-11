package db

import (
	r "gopkg.in/rethinkdb/rethinkdb-go.v6"
)

func CreateUser(user User) (User, error) {
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

func CreateRecipe(recipe Recipe) (Recipe, error) {
	response, err := r.Table(TableNameRecipes).Insert(&recipe).RunWrite(session)
	if err != nil {
		return Recipe{}, err
	}
	// HACK
	id := response.GeneratedKeys[0]
	recipe.ID = id
	return recipe, nil
}

func GetRecipesByUsername(username string) ([]Recipe, error) {
	cursor, err := r.Table(TableNameRecipes).Filter(map[string]interface{}{"ownerId": username}).Run(session)
	if err != nil {
		return nil, err
	}
	defer cursor.Close()
	var recipes []Recipe
	err = cursor.All(&recipes)
	return recipes, err
}

func GetRecipesByUsernameCount(username string) (int, error) {
	cursor, err := r.Table(TableNameRecipes).Filter(map[string]interface{}{"ownerId": username}).Count().Run(session)
	if err != nil {
		return 0, err
	}
	defer cursor.Close()
	var count int
	err = cursor.One(&count)
	return count, err
}
