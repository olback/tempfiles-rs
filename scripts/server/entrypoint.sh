#!/bin/sh

cd /tempfiles

if [[ "$ROCKET_ENV" == "prod" ]]; then

    target/release/tempfiles-rs

elif [[ "$ROCKET_ENV" == "dev" ]]; then

    target/debug/tempfiles-rs

else

    if [ -z "$ROCKET_ENV" ]; then
        echo "Invalid environment: <none>"
    else
        echo "Invalid environment: $ROCKET_ENV"
    fi

    exit 1

fi
