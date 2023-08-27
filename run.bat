@echo off

docker-compose up -d postgres
set DATABASE_URL=postgres://rocket:rocket@localhost:5432/rocket
diesel migration run
cargo run
