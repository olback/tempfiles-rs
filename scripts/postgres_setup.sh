#!/bin/sh

docker exec $(docker ps | grep "$(basename $PWD)_postgres" | rev | cut -d ' ' -f 1 | rev) "/bin/sh" "/scripts/install.sh"
