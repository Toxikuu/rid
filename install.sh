#!/bin/bash
# this script should be run with sudo

[ "$EUID" -ne 0   ]  &&  { echo -e "\x1b[31;1m  Run this script as root\x1b[0m" ; exit 1 ;}

[ -z "$SUDO_USER" ]  &&  SUDO_USER="$TU"
[ -z "$SUDO_USER" ]  &&  { echo -e "\x1b[31;1m  Run this script with sudo or else set $TU=\"<user>\"\x1b[0m" >&2 ; exit 1 ;}

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

export RIDHOME RIDMETA RIDSOURCES
su "$TU" -c '
env -i \
       RIDHOME='"$RIDHOME"'   \
       RIDMETA='"$RIDMETA"'   \
       RIDSOURCES='"$RIDSOURCES"'

PATH="/usr/bin:/usr/sbin:/opt/cargo/bin"
echo "VARS: $RIDHOME $RIDMETA $RIDSOURCES $PATH"

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
cargo strip             || : # in case the user doesnt have cargo strip
'

ln -sfv "$RIDHOME"/rid.sh /usr/bin/rid

if ! grep -q "# rid end" /etc/env ; then
    cat << EOF >> /etc/env

    # rid
    export RIDHOME="$RIDHOME"
    alias rid="sudo -i rid"
    # rid end
EOF
fi

popd    >/dev/null
echo -e "\x1b[36;1m  Done!\x1b[0m"
