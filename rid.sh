#!/bin/bash

export RIDTMP=/tmp/rid
export RIDTRASH="$RIDTMP/trash"
export RIDBUILDING="$RIDTMP/building"
export RIDEXTRACTION="$RIDTMP/extraction"
export RIDDEST="$RIDTMP/dest"
export RIDFAILED="$RIDTMP/failed"

export RIDHOME="/etc/rid"
export RIDMETA="$RIDHOME/meta"
export RIDBIN="$RIDHOME/bin"
export RIDPKGSJSON="$RIDHOME/pkgs.json"
export RIDSOURCES="$RIDHOME/sources"
export RIDSETS="$RIDHOME/sets"

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
