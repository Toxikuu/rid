#!/bin/bash
# meant to be run from rid with `rid -b`
# will build the binary with cargo and rustc if nonexistent

set -e
pushd . >/dev/null
[ -e "$RIDENV"       ]  &&  . "$RIDENV"

[ -z "$RIDHOME"      ]  &&  { RIDHOME="/rid"                                    ;}
[ -z "$RIDMETA"      ]  &&  { RIDMETA="$RIDHOME/meta"                           ;}
[ -z "$RIDSETS"      ]  &&  { RIDMETA="$RIDHOME/sets"                           ;}
[ -z "$RIDBIN"       ]  &&  { RIDBIN="$RIDBIN/bin"                              ;}
[ -z "$RIDSOURCES"   ]  &&  { RIDSOURCES="/sources"                             ;}
[ -e "$RIDPKGSJSON"  ]  &&  { echo "Backing up pkgs.json"           ; BACKUP=1  ;}
[ -n "$BACKUP"       ]  &&  { sudo mv -ivf "$RIDPKGSJSON" /tmp/ridpkgsjson.bak  ;}

echo "Pulling latest changes..."
if [ ! -e "$RIDHOME" ]; then
    git clone "https://github.com/Toxikuu/rid.git" "$RIDHOME"
else
    cd "$RIDHOME" && git pull
fi

if [ ! -e "$RIDMETA" ]; then
    git clone "https://github.com/Toxikuu/rid-meta.git" "$RIDMETA"
else
    cd "$RIDMETA" && git pull
fi
    
echo "Building rid..."
cd   "$RIDHOME"
sudo cargo build --release
sudo cargo strip             || true # in case the user doesn't have cargo strip
sudo ln -sfv                 \
     "$RIDHOME"/rid.sh       \
     /usr/bin/rid

echo 'Setting $RIDENV...'
echo "export RIDENV=$RIDHOME/env" | sudo tee -a /etc/profile > /dev/null

sudo mkdir -pv "$RIDSOURCES"

[ -n "$BACKUP"      ]   &&  { sudo mv -vf /tmp/ridpkgsjson.bak "$RIDPKGSJSON"   ;}

popd    >/dev/null
echo "Done!"
