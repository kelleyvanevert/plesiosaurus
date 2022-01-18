#!/usr/bin/env bash
set -x
set -eo pipefail

if [[ -z "${SKIP_PG_CTL}" ]]; then
    if ! pg_ctl -D /usr/local/var/postgres status > /dev/null; then
        pg_ctl -D /usr/local/var/postgres start > /dev/null;
    fi
fi

export DATABASE_URL=postgres://kelley:password@localhost:${POSTGRES_PORT:=5432}/plesiosaurus

sqlx database create
sqlx migrate run
