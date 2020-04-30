#!/bin/bash

psql -U $POSTGRES_USER -d $POSTGRES_USER -a -f /scripts/sql/tempfiles.psql
psql -U $POSTGRES_USER -d $POSTGRES_USER -a -f /scripts/sql/extensions.psql
