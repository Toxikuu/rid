#!/bin/bash
# this script should be run with sudo

[ "$EUID" -ne 0      ]  &&  { echo "Insufficient permissions"       ; exit 1    ;}

set -e
pushd . >/dev/null
[ -e "$RIDENV"       ]  &&  . "$RIDENV"

[ -z "$RIDHOME"      ]  &&  { RIDHOME="/rid"                                    ;}
[ -z "$RIDMETA"      ]  &&  { RIDMETA="$RIDHOME/meta"                           ;}
[ -z "$RIDSETS"      ]  &&  { RIDMETA="$RIDHOME/sets"                           ;}
[ -z "$RIDBIN"       ]  &&  { RIDBIN="$RIDBIN/bin"                              ;}
[ -z "$RIDSOURCES"   ]  &&  { RIDSOURCES="/sources"                             ;}

echo "Pulling latest changes..."
if [ ! -e "$RIDHOME"/.git ]; then
    git clone "https://github.com/Toxikuu/rid.git" "$RIDHOME"
else
    cd "$RIDHOME" && git pull
fi

if [ ! -e "$RIDMETA"/.git ]; then
    git clone "https://github.com/Toxikuu/rid-meta.git" "$RIDMETA"
else
    cd "$RIDMETA" && git pull
fi
    
echo "Building rid..."
cd   "$RIDHOME"
cargo build --release
cargo strip             || true # in case the user doesn't have cargo strip
ln -sfv                 \
     "$RIDHOME"/rid.sh  \
     /usr/bin/rid

echo 'Setting $RIDENV...'
echo "export RIDENV=$RIDHOME/env" | tee -a /etc/profile > /dev/null

mkdir -pv "$RIDSOURCES"

popd    >/dev/null
echo "Done!"
