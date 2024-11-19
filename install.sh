#!/bin/bash
# meant to be run from rid with `rid -b`
# will build the binary with cargo and rustc if nonexistent

set -e
pushd . >/dev/null
[ -e "$RIDENV"       ]  &&  . "$RIDENV"

[ "$EUID" -ne 0      ]  &&  { echo "Insufficient permissions" >&2   ; exit 1    ;}
[ -z "$RIDHOME"      ]  &&  { RIDHOME="/rid"                                    ;}
[ -z "$RIDMETA"      ]  &&  { RIDMETA="$RIDHOME/meta"                           ;}
[ -z "$RIDSETS"      ]  &&  { RIDMETA="$RIDHOME/sets"                           ;}
[ -z "$RIDBIN"       ]  &&  { RIDBIN="$RIDBIN/bin"                              ;}
[ -z "$RIDSOURCES"   ]  &&  { RIDSOURCES="/sources"                             ;}
[ -e "$RIDPKGSJSON"  ]  &&  { echo "Backing up pkgs.json"           ; BACKUP=1  ;}
[ -n "$BACKUP"       ]  &&  { mv -ivf "$RIDPKGSJSON" /tmp/ridpkgsjson.bak       ;}

mkdir -pv "$RIDSOURCES"

if [ -e "$RIDHOME" ]; then
    cd "$RIDHOME"
    git init
    git remote set-url origin https://github.com/Toxikuu/rid.git
    git pull
else
    git clone https://github.com/Toxikuu/rid.git "$RIDHOME"
fi

if [ -e "$RIDMETA" ]; then
    cd "$RIDMETA"
    git init
    git remote set-url origin https://github.com/Toxikuu/rid-meta.git
    git pull
else
    git clone https://github.com/Toxikuu/rid-meta.git "$RIDMETA"
fi
    
echo "Building rid..."
cd   "$RIDHOME"
cargo build --release
cargo strip             || true # in case the user doesn't have cargo strip
ln -sfv                 \
     "$RIDHOME"/rid.sh  \
     /usr/bin/rid

echo 'Setting $RIDENV...'
echo "export RIDENV=$RIDHOME/env" >> /etc/profile

[ -n "$BACKUP"      ]   &&  { mv -vf /tmp/ridpkgsjson.bak "$RIDPKGSJSON"        ;}

popd    >/dev/null
echo "Done!"
