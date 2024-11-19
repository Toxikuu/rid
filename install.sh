#!/bin/bash
# meant to be run from rid with `rid -b`
# will build the binary with cargo and rustc if nonexistent

set -e
pushd . >/dev/null
[ -e "$RIDENV"       ]  &&  . "$RIDENV"

[ "$EUID" -ne 0      ]  &&  { echo "Insufficient permissions" >&2   ; exit 1    ;}
[ -z "$RIDHOME"      ]  &&  { RIDHOME="/rid"                                    ;}
[ -z "$RIDMETA"      ]  &&  { RIDMETA="$RIDHOME/meta"                           ;}
[ -z "$RIDBIN"       ]  &&  { RIDBIN="$RIDBIN/bin"                              ;}
[ -e "$RIDPKGSJSON"  ]  &&  { echo "Backing up pkgs.json"           ; BACKUP=1  ;}
[ -e "$RIDHOME/.git" ]  &&  { echo "Pulling instead of cloning"     ; PULL=1    ;}
[ -n "$BACKUP"       ]  &&  { mv -ivf "$RIDPKGSJSON" /tmp/ridpkgsjson.bak       ;}

if  [ -z "$PULL" ]; then
    echo "Cloning repositories..."
    git clone https://github.com/Toxikuu/rid.git        "$RIDHOME"
    git clone https://github.com/Toxikuu/rid-meta.git   "$RIDMETA"
    chmod 755                                           "$RIDBIN"/*
else
    echo "Pulling repositories"
    cd   "$RIDHOME"     && git pull
    cd   "$RIDMETA"     && git pull
fi

echo "Building rid..."
cd   "$RIDHOME"
cargo build --release
cargo strip             || true # in case the user doesn't have cargo strip
ln -sfv                 \
     "$RIDHOME"/rid.sh  \
     /usr/bin/rid

echo 'Setting $RIDENV...'
echo "\$RIDENV=$RIDHOME/env" >> /etc/profile

[ -n "$BACKUP"      ]   &&  { mv -vf /tmp/ridpkgsjson.bak "$RIDPKGSJSON"        ;}

popd    >/dev/null
echo "Done!"
