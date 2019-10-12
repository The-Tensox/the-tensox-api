# the-tensox-api
[![Try it on gitpod](https://img.shields.io/badge/try-on%20gitpod-brightgreen.svg)](https://gitpod.io/#https://github.com/The-Tensox/the-tensox-api)
[![Build Status](https://img.shields.io/circleci/project/The-Tensox/the-tensox-api/master.svg)](https://circleci.com/gh/The-Tensox/the-tensox-api)

- REST API to access the-tensox data
- WebSocket server notifying clients on POST / PUT

**Ugly schema**

<img src="docs/images/the-tensox-api.png" width="500">

# Installation
    sudo apt install postgresql-11 libpq-dev
    sudo service postgresql start
    sudo -u postgres psql postgres

Then

    \password postgres

Enter your new password

    CREATE DATABASE "the-tensox";
    \q

Then
```
echo -e "DATABASE_URL=postgres://postgres:mypass@localhost/the-tensox
ROCKET_ADDRESS=localhost
ROCKET_PORT=8001" > .env
```
    cargo install diesel_cli --no-default-features --features postgres
    diesel setup
    diesel migration run
    rustup default nightly # Pear requires a nightly or dev version of Rust
    
# Usage
    cargo run &

    # POST weather
    curl -d '{"id":1, "x":0, "y":0, "sun":3}' -H "Content-Type: application/json" -X POST http://localhost:8001/weathers

    # PUT weather
    curl -d '{"id":1, "x":0, "y":0, "sun":4}' -H "Content-Type: application/json" -X PUT http://localhost:8001/weathers/1
    
    # GET weather
    curl http://localhost:8001/weathers

    # POST object
    curl -d '{"id":1, "x":0, "y":0, "z":0, "scale_x":0, "scale_y":0, "scale_z":0, "height": 0, "radius": 0}' -H "Content-Type: application/json" -X POST http://localhost:8001/objects

    # GET object
    curl http://localhost:8001/objects

# Dev

Update schema

    diesel print-schema > src/schema.rs

## PSQL stuff
Don't forget ; at the end of SQL query (and upper case)

    # Connect psql
    sudo -u postgres psql postgres
    # Switch db
    \c db
    # Show tables
    \dt

# TODO
 - [] When a POST / PUT is done notify the (created / updated) value to the connected clients via websocket