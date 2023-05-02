# Dead Poet Society

### Florian "FloRide" REIMAT

## How to install / launch it

```sh
git clone https://github.com/FloRide1/dead_poet_society.git

# Generate .env file
cp .env.default .env

# Generate self-signed SSL key (don't use for production)
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout ./nginx/cert/nginx.priv.pem -out ./nginx/cert/nginx.pub.pem

docker-compose up -d
```

Please don't forget to setup the env file before launching the docker-compose.
The default .env file is this one [.env.default](./.env.default).
