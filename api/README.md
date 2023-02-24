# My Cooking Codex - API
The api backend.

## Requirements
- RethinkDB Database
- libvips installed

## Environment Variables

| Name         | Description                      | Default   |
| :----------- | :------------------------------- | :-------- |
| HOST         | Host to listen on                | 127.0.0.1 |
| PORT         | Port to bind to                  | 8000      |
| DB__ADDRESS  | DB address with port             |           |
| DB__DB       | Name of database                 |           |
| DB__USERNAME | Username to connect as           |           |
| DB__PASSWORD | Password to use (or leave blank) |           |
| SECRET_KEY   | base64 encoded secret            |           |
