package crud

import (
	"github.com/enchant97/my-cooking-codex/api2/db"
	"github.com/google/uuid"
)

func CreateUser(user db.CreateUser) (db.User, error) {
	var newUser = user.IntoUser()
	if err := db.DB.Create(&newUser).Error; err != nil {
		return db.User{}, err
	}
	return newUser, nil
}

func GetUserByUsername(username string) (db.User, error) {
	var user db.User
	if err := db.DB.First(&user, "username = ?", username).Error; err != nil {
		return db.User{}, err
	}
	return user, nil
}

func GetUserCount() (int64, error) {
	var count int64
	if err := db.DB.Model(&db.User{}).Count(&count).Error; err != nil {
		return 0, err
	}
	return count, nil
}

func CreateRecipe(recipe db.Recipe) (db.Recipe, error) {
	if err := db.DB.Create(&recipe).Error; err != nil {
		return db.Recipe{}, err
	}
	return recipe, nil
}

func GetRecipesByUsername(username string) ([]db.Recipe, error) {
	var recipes []db.Recipe
	if err := db.DB.Find(&recipes, "owner_id = ?", username).Error; err != nil {
		return nil, err
	}
	return recipes, nil
}

func GetRecipesByUsernameCount(username string) (int64, error) {
	var count int64
	if err := db.DB.Model(&db.Recipe{}).Where("owner_id = ?", username).Count(&count).Error; err != nil {
		return 0, err
	}
	return count, nil
}

func GetRecipeById(id string) (db.Recipe, error) {
	var recipe db.Recipe
	if err := db.DB.First(&recipe, "id = ?", id).Error; err != nil {
		return db.Recipe{}, err
	}
	return recipe, nil
}

func DoesUserOwnRecipe(userID uuid.UUID, recipeId string) (bool, error) {
	var recipe db.Recipe
	if err := db.DB.First(&recipe, "id = ?, owner_id = ?", recipeId, userID).Error; err != nil {
		return false, err
	}
	return true, nil
}

func UpdateRecipe(recipeID uint, recipe db.Recipe) (db.Recipe, error) {
	if err := db.DB.Model(&recipe).Where("id = ?", recipeID).Updates(recipe).Error; err != nil {
		return db.Recipe{}, err
	}
	return recipe, nil
}

func DeleteRecipe(recipeID uint) error {
	if err := db.DB.Delete(&db.Recipe{}, recipeID).Error; err != nil {
		return err
	}
	return nil
}
