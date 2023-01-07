package config

import "github.com/caarlos0/env/v6"

// Load the config from OS
func (appConfig *AppConfig) ParseConfig() error {
	if err := env.Parse(appConfig); err != nil {
		return err
	}
	return nil
}
