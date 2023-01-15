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

// set the main recipe image id, if one has not been set already,
// will not check for validity of image id
func SetRecipeMainImageIfUnset(recipeID string, imageID string) error {
	cursor, err := r.Table(TableNameRecipes).Get(recipeID).Pluck("mainImageId").Count().Eq(1).Run(session)
	if err != nil {
		return err
	}
	defer cursor.Close()
	var isSet bool
	if err = cursor.One(&isSet); err != nil {
		return err
	}
	if !isSet {
		_, err := r.Table(TableNameRecipes).Get(recipeID).Update(map[string]interface{}{"mainImageId": imageID}).RunWrite(session)
		return err
	}
	return nil
}

func CreateRecipeImage(recipeImage RecipeImage) (RecipeImage, error) {
	response, err := r.Table(TableNameRecipeImages).Insert(&recipeImage).RunWrite(session)
	if err != nil {
		return RecipeImage{}, err
	}
	// HACK
	id := response.GeneratedKeys[0]
	recipeImage.ID = id
	return recipeImage, nil
}

// Gets the recipe image by it's id without the binary content
func GetRecipeImageWithoutContent(id string) (RecipeImage, error) {
	cursor, err := r.Table(TableNameRecipeImages).Get(id).Without("content").Run(session)
	if err != nil {
		return RecipeImage{}, err
	}
	defer cursor.Close()
	var recipeImage RecipeImage
	err = cursor.One(&recipeImage)
	return recipeImage, err
}

// Gets the recipe image by it's id, including binary content
func GetRecipeImage(id string) (RecipeImage, error) {
	cursor, err := r.Table(TableNameRecipeImages).Get(id).Run(session)
	if err != nil {
		return RecipeImage{}, err
	}
	defer cursor.Close()
	var recipeImage RecipeImage
	err = cursor.One(&recipeImage)
	return recipeImage, err
}

// Gets the recipe image by it's id, returning just the binary content
func GetRecipeImageContentByID(id string) ([]byte, error) {
	cursor, err := r.Table(TableNameRecipeImages).Get(id).Pluck("content").Run(session)
	if err != nil {
		return nil, err
	}
	defer cursor.Close()
	var content []byte
	err = cursor.One(&content)
	return content, err
}

// Gets images for given recipe id, will not include binary content
func GetRecipeImagesByID(recipeID string) ([]RecipeImage, error) {
	cursor, err := r.Table(TableNameRecipeImages).Filter(map[string]interface{}{"recipeId": recipeID}).Without("content").Run(session)
	if err != nil {
		return nil, err
	}
	defer cursor.Close()
	var images []RecipeImage
	err = cursor.All(&images)
	return images, err
}
