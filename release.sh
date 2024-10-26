#!/bin/bash

set -e # exit on error

[ -f src/main.rs ] || {
    echo "Guessing project root: /code/rid"
    pushd /code/rid
}

XZ_OPT="-9e"

echo "Packaging rid-offline"
cargo build --release --no-default-features --features=offline
pushd target/release
mv -vf rid{,-offline}
tar cJf rid-offline.tar.xz rid-offline

echo "Packaging rid"
cargo build --release
tar cJf rid.tar.xz rid

popd > /dev/null
popd > /dev/null 2>&1 || true # in case project root was guessed
