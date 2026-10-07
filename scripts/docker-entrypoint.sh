#!/bin/sh
# Panel container entrypoint (v1.2.12): start as root only long enough to fix
# ownership, then run the panel as the unprivileged `relaypanel` user.
#
#   /app/run   the one-click update's request/status directory (bind mount of
#              the host's ./run). The panel only needs to create files in it,
#              so only the mount point itself changes owner — not recursively,
#              since the host updater writes files there as root.
#
# Started with `--user` / `user:` (not root): nothing to fix, run as given.

set -eu

if [ "$(id -u)" = "0" ]; then
    if [ -d /app/run ]; then
        chown relaypanel:relaypanel /app/run
    fi
    exec setpriv --reuid=relaypanel --regid=relaypanel --init-groups -- "$@"
fi

exec "$@"
