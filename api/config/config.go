package config

type DBConfig struct {
	Address  string `env:"ADDRESS,required"`
	Database string `env:"DB,required"`
	Username string `env:"USERNAME,required"`
	Password string `env:"PASSWORD,required"`
}

type AppConfig struct {
	Host      string        `env:"HOST" envDefault:"127.0.0.1"`
	Port      uint          `env:"PORT" envDefault:"8000"`
	DB        DBConfig      `envPrefix:"DB__"`
	SecretKey Base64Decoded `env:"SECRET_KEY,notEmpty"`
}
