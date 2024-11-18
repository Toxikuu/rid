#!/bin/bash

set -e

[ -f src/main.rs ] || {
    echo "Guessing project root: /code/rid"
    pushd /code/rid > /dev/null
}


echo -e "\x1b[36;1m  Fixing version in README...\x1b[0m"
RIDVERS=$(rg -oN '\d*\.\d*\.\d*\d' Cargo.toml | head -n1)
sed -i "s/[0-9]*\.[0-9]*\.[0-9]*[0-9]/$RIDVERS/" README.md

pushd target/release > /dev/null
XZ_OPT="-9e"

echo -e "\x1b[36;1m  Removing old releases...\x1b[0m"
rm -vf rid*

echo -e "\x1b[36;1m  Packaging rid...\x1b[0m"
cargo build --release
echo "Compressing rid"
tar cJvf rid.tar.xz rid > /dev/null

echo -e "\x1b[36;1m  Packaging project root...\x1b[0m"
echo "Compressing rid-root"
tar cJvf rid-root.tar.xz ../../bin ../../sets ../../env ../../rid.sh ../../{README,DOCS}.md > /dev/null 2>&1 # suppress a tar warning about leading ../

popd > /dev/null
popd > /dev/null 2>&1 || true # in case project root was guessed
echo -e "\x1b[36;1m  Done!\x1b[0m"
