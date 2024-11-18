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

# rid-offline is now deprecated
# echo -e "\x1b[36;1m  Packaging rid-offline...\x1b[0m"
# cargo build --release --no-default-features --features=offline
# mv -vf rid{,-offline}
# echo "Compressing rid-offline"
# tar cJvf rid-offline.tar.xz rid-offline > /dev/null
# echo "Done"

echo -e "\x1b[36;1m  Packaging rid...\x1b[0m"
cargo build --release
echo "Compressing rid"
tar cJvf rid.tar.xz rid > /dev/null
echo "Done"

echo -e "\x1b[36;1m  Packaging project root...\x1b[0m"
echo "Compressing rid-root"
tar cJvf rid-root.tar.xz ../../bin ../../sets ../../env ../../rid.sh ../../{README,DOCS}.md > /dev/null 2>&1 # suppress a tar warning about leading ../
echo "Done"

popd > /dev/null
popd > /dev/null 2>&1 || true # in case project root was guessed
echo -e "\x1b[36;1m  Done!\x1b[0m"
