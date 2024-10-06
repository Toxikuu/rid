#!/bin/bash
NAME="nss"
VERS=3.105
ver="${VERS//./_}"
LINK="https://archive.mozilla.org/pub/security/nss/releases/NSS_${ver}_RTM/src/nss-$VERS.tar.gz"
DEPS="nspr"


IDIR=$(cat <<EOF

raw https://www.linuxfromscratch.org/patches/blfs/svn/nss-standalone-1.patch
patch -Np1 -i /etc/rid/sources/nss-standalone-1.patch &&
cd nss &&

make BUILD_OPT=1                      \
  NSPR_INCLUDE_DIR=/usr/include/nspr  \
  NSS_DISABLE_GTESTS=1                \
  USE_SYSTEM_ZLIB=1                   \
  ZLIB_LIBS=-lz                       \
  NSS_ENABLE_WERROR=0                 \
  $([ $(uname -m) = x86_64 ] && echo USE_64=1) \
  $([ -f /usr/include/sqlite3.h ] && echo NSS_USE_SYSTEM_SQLITE=1)

cd ../dist                                                          &&

install -v -m755 Linux*/lib/*.so              /usr/lib              &&
install -v -m644 Linux*/lib/{*.chk,libcrmf.a} /usr/lib              &&

install -v -m755 -d                           /usr/include/nss      &&
cp -v -RL {public,private}/nss/*              /usr/include/nss      &&

install -v -m755 Linux*/bin/{certutil,nss-config,pk12util} /usr/bin &&

install -v -m644 Linux*/lib/pkgconfig/nss.pc  /usr/lib/pkgconfig

cd ..
rm -rvf nss-$VERS
tar xvf /etc/rid/sources/nss-$VERS.t*

cd nss-$VERS &&
patch -Np1 -i /etc/rid/sources/nss-standalone-1.patch &&
cd nss &&

CC="gcc -m32" CXX="g++ -m32"          \
make BUILD_OPT=1                      \
  NSPR_INCLUDE_DIR=/usr/include/nspr  \
  USE_SYSTEM_ZLIB=1                   \
  NSS_DISABLE_GTESTS=1                \
  ZLIB_LIBS=-lz                       \
  NSS_ENABLE_WERROR=0                 \
$([ -f /usr/lib32/libsqlite3.so ] && echo NSS_USE_SYSTEM_SQLITE=1)

cd ../dist &&

install -v -m755 Linux*/lib/*.so              /usr/lib32              &&
install -v -m644 Linux*/lib/{*.chk,libcrmf.a} /usr/lib32              &&
sed -i 's/lib/lib32/g' Linux*/lib/pkgconfig/nss.pc                    &&
install -v -m644 Linux*/lib/pkgconfig/nss.pc  /usr/lib32/pkgconfig

EOF
)

RDIR=$(cat <<EOF

echo not implemented

EOF
)

UDIR=$(cat <<EOF
EOF
)
