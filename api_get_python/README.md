# API Get Python

### By Florian "FloRide" Reimat

This is a simple API that allow GET request to view data from dead_poet_society database.
It's build on Python with [Flask](https://pypi.org/project/Flask) as the backend framework.

## How to launch it

**Examples environnement variables:**

```env
POSTGRES_USER="postgres"                # Database user connection
POSTGRES_PASSWORD="postgres"            # Database password connection
POSTGRES_PORT=5432                      # Database port
POSTGRES_HOST="localhost"               # Database host
POSTGRES_DB="dead_poet_society_db"      # Database name

FLASK_ENV="production"                  # Flask environnement type
FLASK_RUN_PORT=5001                     # Flask listening port
```

**Run commands:**

```sh
# Setup .env
touch .env
$EDITOR .env # Use your favorite editor

# Install required package
pip -r ./requirements.txt

# Debug | Run
python ./main.py

# Docker
docker build -t dead_poet_society_python_get_api .
docker run -t dead_poet_society_python_get_api -d
```

## Documentation

- OpenAPI:
  - Download / Copy [open-api.yml](./misc/open-api.yml)
  - Use it on [swagger.io](https://editor.swagger.io)
