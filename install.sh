#!/bin/bash
# meant to be run from rid with `rid -b`
# will build the binary with cargo and rustc if nonexistent

set -e
pushd . >/dev/null

[ "$EUID" -ne 0     ]   &&  { echo "Insufficient permissions" >&2   ; exit 1    ; }
[ -z "$RIDSOURCES"  ]   &&  { echo '$RIDSOURCES not set'            ; exit 1    ; }
[ -z "$RIDMETA"     ]   &&  { echo '$RIDMETA not set'               ; exit 1    ; }
[ -z "$RIDBIN"      ]   &&  { echo '$RIDBIN not set'                ; exit 1    ; }
[ -e "$RIDPKGSJSON" ]   &&  { echo "Backing up pkgs.json"           ; BACKUP=1  ; }
[ -z "$BACKUP"      ]   &&  { mv -vf "$RIDPKGSJSON" /tmp/ridpkgsjson.bak        ; }

echo "Cloning repositories..."
git clone https://github.com/Toxikuu/rid.git                        "$RIDHOME"
git clone https://github.com/Toxikuu/rid-meta.git                   "$RIDMETA"
chmod 755                                                           "$RIDBIN"/*

[ -z "$BACKUP"      ]   &&  { mv -vf /tmp/ridpkgsjson.bak "$RIDPKGSJSON"        ; }

popd    >/dev/null
echo "Done!"
