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
echo -e "DATABASE_URL=postgres://postgres:mypass@localhost/the-tensox-api
ROCKET_ADDRESS=localhost
ROCKET_PORT=8001" > .env
```
    cargo install diesel_cli --no-default-features --features postgres
    diesel migration run
    rustup default nightly # Pear requires a nightly or dev version of Rust
    cargo run

Open another terminal

    curl -d '{"id":1, "x":0, "y":0, "sun":3}' -H "Content-Type: application/json" -X POST http://localhost:8001/weather

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