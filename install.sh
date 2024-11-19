#!/bin/bash
# this script should be run with sudo

[ "$EUID" -ne 0   ]  &&  { echo -e "\x1b[31;1m  Run this script as root\x1b[0m" ; exit 1 ;}

[ -z "$SUDO_USER" ]  &&  SUDO_USER="$TARGET_USER"
[ -z "$SUDO_USER" ]  &&  { echo -e "\x1b[31;1m  Run this script with sudo :)\x1b[0m" >&2 ; exit 1 ;}

TU="$SUDO_USER"
[ -z "$TU" ] && { echo -e "\x1b[31;1m  Could not determine target user!\x1b[0m" >&2; exit 1; }

set -e
pushd . >/dev/null
pushd . >/dev/null

[ -z "$RIDHOME"      ]  &&  { RIDHOME="/rid"            ;}
[ -z "$RIDMETA"      ]  &&  { RIDMETA="/var/rid/meta"   ;}
[ -z "$RIDSOURCES"   ]  &&  { RIDSOURCES="/sources"     ;}

mkdir -pv "$RIDHOME" "$RIDMETA"
chown -R  "$TU:$TU"  "$RIDHOME" "$RIDMETA"

su -p "$TU" -c '
PATH="/usr/bin:/usr/sbin:/opt/cargo/bin"

echo -e "\x1b[36;1m  Pulling latest changes...\x1b[0m"
if [ ! -e "$RIDHOME"/.git ]; then
    git clone "https://github.com/Toxikuu/rid.git" "$RIDHOME"
else
    cd "$RIDHOME" && git pull
fi
echo "Pulled for RIDHOME ($RIDHOME)"

if [ ! -e "$RIDMETA"/.git ]; then
    git clone "https://github.com/Toxikuu/rid-meta.git" "$RIDMETA"
else
    cd "$RIDMETA" && git pull
fi
echo "Pulled for RIDMETA ($RIDMETA)"

echo -e "\x1b[36;1m  Building rid...\x1b[0m"
cd   "$RIDHOME"
cargo build --release
cargo strip             || true # in case the user doesnt have cargo strip
'

ln -sfv "$RIDHOME"/rid.sh /usr/bin/rid
grep -qxF "export RIDHOME=$RIDHOME" /etc/profile ||
     echo "export RIDHOME=$RIDHOME" >> /etc/profile

grep -qxF "alias rid='sudo -i rid'" /etc/profile ||
     echo "alias rid='sudo -i rid'" >> /etc/profile

popd    >/dev/null
echo -e "\x1b[36;1m  Done!\x1b[0m"
