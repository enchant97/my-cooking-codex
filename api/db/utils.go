package db

import (
	"github.com/enchant97/my-cooking-codex/api/config"
	r "gopkg.in/rethinkdb/rethinkdb-go.v6"
)

var session *r.Session

func ensureTableCreated(tableName string) error {
	cursor, err := r.TableList().Contains(tableName).Not().Run(session)
	if err != nil {
		return err
	}
	defer cursor.Close()
	var needsCreation bool
	err = cursor.One(&needsCreation)
	if err != nil {
		return err
	}
	if needsCreation {
		if _, err = r.TableCreate(tableName).RunWrite(session); err != nil {
			return err
		}
	}
	return nil
}

func InitDB(config config.DBConfig) error {
	var err error
	session, err = r.Connect(r.ConnectOpts{
		Address:  config.Address,
		Database: config.Database,
		Username: config.Username,
		Password: config.Password,
	})
	if err != nil {
		return err
	}
	if err = ensureTableCreated(TableNameUsers); err != nil {
		return err
	}
	if err = ensureTableCreated(TableNameRecipes); err != nil {
		return err
	}
	if err = ensureTableCreated(TableNameRecipeImages); err != nil {
		return err
	}
	return nil
}

func CloseDB() error {
	return session.Close()
}
