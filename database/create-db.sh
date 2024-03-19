#!/bin/bash

docker stop pg-grocery-list
docker rm pg-grocery-list 
docker run --name pg-grocery-list -p 5432:5432 -e POSTGRES_USER=admin -e POSTGRES_PASSWORD=admin123 -e POSTGRES_DB=grocery_list -v ./init.sql:/docker-entrypoint-initdb.d/init.sql -v ./snapshots:/home/snapshots -d postgres