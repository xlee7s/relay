#!/usr/bin/env bash
# Consistent logical backup of the bundled MySQL 5.7 database.
set -euo pipefail

COMPOSE_FILE="${1:?usage: mysql-backup.sh <compose-file> <label>}"
LABEL="${2:?usage: mysql-backup.sh <compose-file> <label>}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

container="$(docker compose -f "$COMPOSE_FILE" ps -q mysql 2>/dev/null || true)"
if [ -z "$container" ] || [ "$(docker inspect -f '{{.State.Running}}' "$container" 2>/dev/null || true)" != "true" ]; then
    exit 3
fi

mkdir -p backups
stamp="$(date -u +%Y%m%d-%H%M%S)"
target="backups/mysql-${stamp}-${LABEL}.sql.gz"
tmp="${target}.tmp"
trap 'rm -f "$tmp"' EXIT

docker compose -f "$COMPOSE_FILE" exec -T mysql sh -c \
    'exec mysqldump --single-transaction --quick --routines --triggers --default-character-set=utf8mb4 -uroot -p"$MYSQL_ROOT_PASSWORD" "$MYSQL_DATABASE"' \
    | gzip -9 > "$tmp"

test -s "$tmp"
mv "$tmp" "$target"
trap - EXIT

ls -1t backups/mysql-*.sql.gz 2>/dev/null | awk 'NR>5' | while IFS= read -r old; do
    rm -f -- "$old"
done

printf '%s\n' "$target"
