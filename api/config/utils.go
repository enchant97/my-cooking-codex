package config

import "github.com/caarlos0/env/v6"

type AppConfig struct {
	DB DBConfig `envPrefix:"DB__"`
}

// Load the config from OS
func (appConfig *AppConfig) ParseConfig() error {
	if err := env.Parse(appConfig); err != nil {
		return err
	}
	return nil
}
