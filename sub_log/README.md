# API Edit Rust

### By Florian "FloRide" Reimat

This is a simple sublog system using MQTT as the main source of event
It's build on Rust with the [Rumqttc](https://crates.io/crates/rumqttc) crate as the MQTT client.

## How to launch it

**Examples environnement variables:**

```env
MQTT_HOST="localhost"   # The MQTT host server
MQTT_PORT=1883          # The MQTT port server
RUST_LOG="info"         # The log level (trace | debug | info | error | off)
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
docker build -t dead_poet_society_sublog .
docker run -t dead_poet_society_sublog -d
```

## Documentation

- MQTT Client Subscribe to:

  - `new_writer_json`
  - `new_writer_confirmed_json`
  - `edit_writer_json`
  - `edit_writer_confirmed_json`
  - `delete_writer_json`
  - `delete_writer_confirmed_json`

  - `new_circle_json`
  - `new_circle_confirmed_json`
  - `edit_circle_json`
  - `edit_circle_confirmed_json`
  - `delete_circle_json`
  - `delete_circle_confirmed_json`

  - `new_letter_json`
  - `new_letter_confirmed_json`
  - `delete_letter_json`
  - `delete_letter_confirmed_json`
