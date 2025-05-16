#!/bin/bash

PASSWORD=$1

docker pull postgres

docker run --name postgres_receptor_db -v /var/lib/postgresql:/var/lib/postgresql -e POSTGRES_PASSWORD=${PASSWORD} -it -p 3306:3306 -d postgres
