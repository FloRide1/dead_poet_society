# API Edit Rust

### By Florian "FloRide" Reimat

This is a simple API that allow POST, PATCH, DELETE request to edit dead_poet_society database.
It's build on Rust with [Rocket](https://rocket.rs) as the backend framework and [Diesel](https://diesel.rs) as the ORM.
It's also linked to a MQTT server, were everytime a request is received, it publish to a the MQTT server.

## How to launch it

**Examples environnement variables:**

```env
POSTGRES_USER="postgres"                                # Database user connection
POSTGRES_PASSWORD="postgres"                            # Database password connection
POSTGRES_PORT=5432                                      # Database port
POSTGRES_HOST="localhost"                               # Database host
POSTGRES_DB="dead_poet_society_db"                      # Database name

DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"

ROCKET_ADDRESS="0.0.0.0"                                # The server address
ROCKET_PORT=8000                                        # The server port
ROCKET_LOG_LEVEL="normal"                               # The server log level (off | normal | debug | critical)
ROCKET_DATABASES="{diesel={url=\"${DATABASE_URL}\"}}"   # The database link connection (don't touch it)

MQTT_HOST="localhost"                                   # The MQTT host server
MQTT_PORT=1883                                          # The MQTT port server
```

**Run commands:**

```sh
# Setup .env
touch .env
$EDITOR .env # Use your favorite editor

# Debug | Run
cargo run

# Build
cargo build --release

# Docker
docker build -t dead_poet_society_rust_edit_api .
docker run -t dead_poet_society_rust_edit_api -d
```

## Documentation

- OpenAPI:
  - Download / Copy [open-api.yml](./misc/open-api.yml)
  - Use it on [swagger.io](https://editor.swagger.io)
