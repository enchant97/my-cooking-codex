package config

type DBConfig struct {
	Address  string `env:"ADDRESS,required"`
	Database string `env:"DB,required"`
	Username string `env:"USERNAME,required"`
	Password string `env:"PASSWORD,required"`
}
