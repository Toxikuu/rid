#!/bin/bash

REPO="${REPO:-main}" . "$RIDHOME"/env || die "Failed to source dev rid's env"
# echo "[rid.sh] REPO: $REPO" # debugging

"$RIDHOME"/target/release/rid "$@"
