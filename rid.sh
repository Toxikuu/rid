#!/bin/bash

REPO="${REPO:-main}" . "$RIDHOME"/env || die "Failed to source dev rid's env"

# the below is necessary to remove leftover source directories from packages that do not have links
# TODO: move this functionality to rid
[ -z "$RIDBUILDING" ] && die '$RIDBUILING not set'
[ -z "$RIDHOME" ]     && die '$RIDHOME not set'
# rm -rf "$RIDBUILDING"/*

"$RIDHOME"/target/release/rid "$@"
