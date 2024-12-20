#!/bin/bash

set -e

# build a release binary
cargo build --release

# build a binary optimized for size
# cargo build --profile size
# upx --best --lzma target/size/rid

# package
pushd target/release
XZ_OPT=-9e tar cJvf rid.tar.xz rid
popd

# don't bother packaging the size binary, since doing so saves only about 40 bytes
# pushd target/size
# XZ_OPT=-9e tar cJvf rid.tar.xz rid
# popd
