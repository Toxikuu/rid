#!/bin/bash
# meant to be run from rid with `rid -b`
# will build the binary with cargo and rustc if nonexistent

set -e

[ -e /usr/bin/rid ] || BUILDRID=1

echo "Extracting rid-root tarball..."
tar xf "$RIDSOURCES"/rid-root.tar.xz -C "$RIDHOME"

if ! [ -z "$BUILDRID" ]; then
    echo "Building rid..."
    pushd "$RIDHOME" > /dev/null
    ./release.sh
    ln -sfv $PWD/target/release/rid /usr/bin/rid
    popd
fi

echo "Extracting rid-meta tarball..."
tar xf "$RIDSOURCES"/master.tar.gz -C "$RIDMETA"

echo "Adjusting permissions..."
chmod 755 "$RIDBIN"/*

echo "Done!"
