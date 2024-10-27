#!/bin/bash

set -e # exit on error

[ -f src/main.rs ] || {
    echo "Guessing project root: /code/rid"
    pushd /code/rid > /dev/null
}

pushd target/release
XZ_OPT="-9e"

echo -e "\x1b[36;1m  Removing old releases...\x1b[0m"
rm -vf rid*

echo -e "\x1b[36;1m  Packaging rid-offline...\x1b[0m"
cargo build --release --no-default-features --features=offline
mv -vf rid{,-offline}
tar cJvf rid-offline.tar.xz rid-offline

echo -e "\x1b[36;1m  Packaging rid...\x1b[0m"
cargo build --release
tar cJvf rid.tar.xz rid

echo -e "\x1b[36;1m  Packaging project root...\x1b[0m"
tar cJvf rid-root.tar.xz ../../rbin ../../env

popd > /dev/null
popd > /dev/null 2>&1 || true # in case project root was guessed
echo -e "\x1b[36;1m  Done!\x1b[0m"
