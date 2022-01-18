#!/usr/bin/env bash
set -x
set -eo pipefail

if ! pg_ctl -D /usr/local/var/postgres status > /dev/null; then
    pg_ctl -D /usr/local/var/postgres start > /dev/null;
fi;

export DATABASE_URL=postgres://kelley:@localhost:5432/plesiosaurus

sqlx database create
sqlx migrate run
