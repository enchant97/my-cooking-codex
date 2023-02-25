# My Cooking Codex - API
The api backend.

## Requirements
- SQLite/MySQL/PostgreSQL Database
- libvips installed

## Environment Variables

| Name       | Description                                    | Default   |
| :--------- | :--------------------------------------------- | :-------- |
| HOST       | Host to listen on                              | 127.0.0.1 |
| PORT       | Port to bind to                                | 8000      |
| DATA_PATH  | Where app data will be stored                  |           |
| DB__URI    | Database URI                                   |           |
| DB__TYPE   | The type of database (sqlite, mysql, postgres) |           |
| SECRET_KEY | base64 encoded secret                          |           |

### DB__URI

```
# sqlite
./data/db.sqlite

# mysql
user:pass@tcp(127.0.0.1:3306)/my_cooking_codex?charset=utf8mb4&parseTime=True&loc=Local

# postgres
host=localhost user=user password=password dbname=my_cooking_codex port=9920 sslmode=disable TimeZone=Europe/London
```
