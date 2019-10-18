# the-tensox-api
[![Try it on gitpod](https://img.shields.io/badge/try-on%20gitpod-brightgreen.svg)](https://gitpod.io/#https://github.com/The-Tensox/the-tensox-api)
[![Build Status](https://img.shields.io/circleci/project/The-Tensox/the-tensox-api/master.svg)](https://circleci.com/gh/The-Tensox/the-tensox-api)

- REST API to access the-tensox data
- WebSocket server notifying clients on POST / PUT

**Ugly schema**

<img src="docs/images/the-tensox-api.png" width="500">

## Installation

```bash
sudo apt install postgresql-11 libpq-dev
sudo service postgresql start
sudo -u postgres psql postgres
```

Then

```bash
\password postgres
```

Enter your new password

```sql
CREATE DATABASE "the-tensox";
\q
```

Then

```bash
echo -e "DATABASE_URL=postgres://postgres:mypass@localhost/the-tensox
ROCKET_ADDRESS=localhost
ROCKET_PORT=8001" > .env
```

```bash
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration run
rustup default nightly # Pear requires a nightly or dev version of Rust
```

## Usage

```bash
cargo run &

# POST object
# Empty
curl -d '{"id": 1}' -H "Content-Type: application/json" -X POST http://localhost:8001/objects
# Or
curl -d '{"id":1, "position_x":0, "position_y":0, "position_z":0,
"rotation_x":0, "rotation_y":0, "rotation_z":0, "scale_x":0, "scale_y":0,
"scale_z":0, "mass": 0, "velocity_x": 0, "velocity_y": 0, "velocity_z": 0,
"collision_x": 0, "collision_y": 0, "collision_z": 0, "height": 0, "radius": 0, "kind": "rock"}' \
-H "Content-Type: application/json" -X POST http://localhost:8001/objects

# GET object
curl http://localhost:8001/objects
```

## Dev

Update schema

    diesel print-schema > src/schema.rs

### PSQL stuff
Don't forget ; at the end of SQL query (and upper case)

    # Connect psql
    sudo -u postgres psql postgres
    # Switch db
    \c db
    # Show tables
    \dt

## Contribute

- Use cargo fmt

## TODO

 - [x] When a POST / PUT is done notify the (created / updated) value to the connected clients via websocket
 - [ ] Benches [see example](https://bheisler.github.io/criterion.rs/book/getting_started.html)
 - [ ] Tests
