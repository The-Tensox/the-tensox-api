
# the-tensox-api

[![Try it on gitpod](https://img.shields.io/badge/try-on%20gitpod-brightgreen.svg)](https://gitpod.io/#https://github.com/The-Tensox/the-tensox-api)
[![Build Status](https://img.shields.io/circleci/project/The-Tensox/the-tensox-api/master.svg)](https://circleci.com/gh/The-Tensox/the-tensox-api)

- REST API to access the-tensox data
- WebSocket server notifying clients on POST / PUT

## Installation

```bash
sudo apt update
sudo apt install mongodb
```

Check mongodb status

```bash
service mongodb status
```

```bash
echo -e "MONGO_ADDR=localhost
DB_NAME=rustlang-rocket-mongodb
MONGO_PORT=27017" > .env
```

## Usage

```bash
cargo run &

# POST
curl -d '{"position_x": 13}' -H "Content-Type: application/json" -X POST http://localhost:8001/objects
# Or
curl -d '{"mesh": {"Array": {"meshes": [{"normals": [0.0, 0.0, 0.0], "uvs": [0.0, 0.0], "vertices": [1.0, 1.0, 1.0]}]}}}' \
-H "Content-Type: application/json" -X POST http://localhost:8001/objects
# Or empty
curl -d '{}' -H "Content-Type: application/json" -X POST http://localhost:8001/objects

# PUT
curl -d '{"$oid": "5db15a686539303d5708901f", "position_y": 27}' -H "Content-Type: application/json" \
-X PUT http://localhost:8001/objects/5db15a686539303d5708901f

# GET
curl http://localhost:8001/objects
# Find by id
curl http://localhost:8001/objects/5db15a1f6539303d5708901e

# DELETE
curl -H "Content-Type: application/json" -X DELETE http://localhost:8001/objects/5db15a1f6539303d5708901e

# DELETE all
curl -X DELETE localhost:8001/objects
```

## Tests

To avoid running parallel tests we use --test-threads=1 because we modify database, otherwise tests would fail.

```rust
cargo test -- --test-threads=1
```

## Contribute

- Use cargo fmt

## TODO

 - [x] When a POST / PUT is done notify the (created / updated) value to the connected clients via websocket
 - [ ] Benches [see example](https://bheisler.github.io/criterion.rs/book/getting_started.html)
 - [x] Tests
 - [ ] Documentation [see](https://doc.rust-lang.org/rust-by-example/meta/doc.html)