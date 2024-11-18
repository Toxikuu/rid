#!/bin/bash

SCRIPT_DIR=$(dirname "$(realpath "$0")")
. $SCRIPT_DIR/env

if [[ $EUID -ne 0 || ! $(shopt -q login_shell; echo $?) -eq 0 ]]; then
  CMD=$(command -v sudo > /dev/null 2>&1 && echo "sudo su - root" || echo "su - root")
  exec $CMD -c "
    export RIDTMP=\$RIDTMP;
    export RIDTRASH=\$RIDTRASH;
    export RIDBUILDING=\$RIDBUILDING;
    export RIDEXTRACTION=\$RIDEXTRACTION;
    export RIDDEST=\$RIDDEST;
    export RIDFAILED=\$RIDFAILED;
    export RIDHOME=\$RIDHOME;
    export RIDMETA=\$RIDMETA;
    export RIDBIN=\$RIDBIN;
    export RIDPKGSJSON=\$RIDPKGSJSON;
    export RIDSOURCES=\$RIDSOURCES;
    export RIDSETS=\$RIDSETS;
    rid $*
  "
  exit 0
fi

rid "$@"
