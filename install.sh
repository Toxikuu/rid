#!/bin/bash
# this script should be run with sudo

[ -z "$SUDO_USER"    ]  &&  { echo -e "\x1b[31;1m  Run this script with sudo :)\x1b[0m" >&2 ; exit 1 ;}

set -e
pushd . >/dev/null
PATH="/usr/bin:/usr/sbin:/opt/cargo/bin"

[ -z "$RIDHOME"      ]  &&  { RIDHOME="/rid"            ;}
[ -z "$RIDMETA"      ]  &&  { RIDMETA="$RIDHOME/meta"   ;}
[ -z "$RIDSOURCES"   ]  &&  { RIDSOURCES="/sources"     ;}

echo -e "\x1b[36;1m  Pulling latest changes...\x1b[0m"
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
    
echo -e "\x1b[36;1m  Building rid...\x1b[0m"
cd   "$RIDHOME"
cargo build --release
cargo strip             || true # in case the user doesn't have cargo strip
ln -sfv                 \
     "$RIDHOME"/rid.sh  \
     /usr/bin/rid

echo -e "\x1b[36;1m  Writing to /etc/profile...\x1b[0m"
echo "export RIDHOME=$RIDHOME" | tee -a /etc/profile > /dev/null

mkdir -pv "$RIDSOURCES"

popd    >/dev/null
sudo chown -R $SUDO_USER:$SUDO_USER $RIDHOME # necessary if you want to git pull (this isn't very secure)
echo -e "\x1b[36;1m  Done!\x1b[0m"
