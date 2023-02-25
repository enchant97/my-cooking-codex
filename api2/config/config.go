package config

type DBConfig struct {
	SQLitePath string `env:"SQLITE_PATH,notEmpty"`
}

type AppConfig struct {
	Host      string        `env:"HOST" envDefault:"127.0.0.1"`
	Port      uint          `env:"PORT" envDefault:"8000"`
	DB        DBConfig      `envPrefix:"DB__"`
	SecretKey Base64Decoded `env:"SECRET_KEY,notEmpty"`
}
