#!/bin/bash

docker-compose up --build -d postgres
DATABASE_URL=postgres://rocket:rocket@localhost:5432/rocket 
diesel migration run
cargo watch -q -c -w src/ -x run
