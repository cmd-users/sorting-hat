#!/bin/bash

docker-compose up -d postgres
DATABASE_URL=postgres://rocket:rocket@localhost:5432/rocket 
diesel migration run
cargo run
