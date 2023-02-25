package db

import (
	"github.com/enchant97/my-cooking-codex/api2/config"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

var DB *gorm.DB

func InitDB(conf config.DBConfig) error {
	db, err := gorm.Open(sqlite.Open(conf.SQLitePath), &gorm.Config{})
	if err != nil {
		return err
	}
	DB = db
	return nil
}
