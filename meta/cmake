#!/bin/bash
NAME="cmake"
VERS=3.30.4
ver="${VERS%.*}"
LINK="https://cmake.org/files/v$ver/cmake-$VERS.tar.gz"
DEPS="curl libarchive libuv nghttp2"


IDIR=$(cat <<EOF

sed -i '/"lib64"/s/64//' Modules/GNUInstallDirs.cmake &&

./bootstrap --prefix=/usr          \
            --parallel=$(nproc)    \
            --system-libs          \
            --mandir=/share/man    \
            --no-system-jsoncpp    \
            --no-system-cppdap     \
            --no-system-librhash   \
            --docdir=/share/doc/cmake-$VERS &&
make
make-install

EOF
)

RDIR=$(cat <<EOF

echo not implemented

EOF
)

UDIR=$(cat <<EOF
EOF
)
