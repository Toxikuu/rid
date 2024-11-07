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
export RIDPKGSVERS="$RIDMETA/VERS"
export RIDSOURCES="$RIDHOME/sources"
export RIDSETS="$RIDHOME/sets"

rid "$@"
