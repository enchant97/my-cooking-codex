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

func GetRecipeById(id string, username string) (Recipe, error) {
	cursor, err := r.Table(TableNameRecipes).Filter(map[string]interface{}{"id": id, "ownerId": username}).Run(session)
	if err != nil {
		return Recipe{}, err
	}
	defer cursor.Close()
	var recipe Recipe
	err = cursor.One(&recipe)
	return recipe, err
}

func DoesUserOwnRecipe(username string, recipeID string) (bool, error) {
	cursor, err := r.Table(TableNameRecipes).Filter(map[string]interface{}{"id": recipeID, "ownerId": username}).IsEmpty().Not().Run(session)
	if err != nil {
		return false, err
	}
	defer cursor.Close()
	var isOwner bool
	err = cursor.One(&isOwner)
	return isOwner, err
}

func UpdateRecipe(recipeID string, newRecipe interface{}) error {
	_, err := r.Table(TableNameRecipes).Get(recipeID).Update(newRecipe).RunWrite(session)
	return err
}

func SetRecipeImage(recipeImage RecipeImage) (RecipeImage, error) {
	_, err := r.Table(TableNameRecipeImages).Insert(&recipeImage, r.InsertOpts{Conflict: "replace"}).RunWrite(session)
	if err != nil {
		return RecipeImage{}, err
	}
	return recipeImage, nil
}

// Gets the recipe image by it's id without the binary content
func GetRecipeImageWithoutContent(recipeID string) (RecipeImage, error) {
	cursor, err := r.Table(TableNameRecipeImages).Get(recipeID).Without("content").Run(session)
	if err != nil {
		return RecipeImage{}, err
	}
	defer cursor.Close()
	var recipeImage RecipeImage
	err = cursor.One(&recipeImage)
	return recipeImage, err
}

// Gets the recipe image by it's id, including binary content
func GetRecipeImage(recipeID string) (RecipeImage, error) {
	cursor, err := r.Table(TableNameRecipeImages).Get(recipeID).Run(session)
	if err != nil {
		return RecipeImage{}, err
	}
	defer cursor.Close()
	var recipeImage RecipeImage
	err = cursor.One(&recipeImage)
	return recipeImage, err
}

// Gets the recipe image by it's id, returning just the binary content
func GetRecipeImageContent(recipeID string) ([]byte, error) {
	cursor, err := r.Table(TableNameRecipeImages).Get(recipeID).Pluck("content").Run(session)
	if err != nil {
		return nil, err
	}
	defer cursor.Close()
	var content []byte
	err = cursor.One(&content)
	return content, err
}

func DeleteRecipeImage(recipeID string) error {
	_, err := r.Table(TableNameRecipeImages).Get(recipeID).Delete().RunWrite(session)
	return err
}
