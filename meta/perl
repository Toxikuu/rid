#!/bin/bash
NAME="perl"
VERS=5.40.0
ver="${VERS%.*.*}"
LINK="https://www.cpan.org/src/$ver.0/perl-$VERS.tar.xz"
DEPS=""


IDIR=$(cat <<EOF

export BUILD_ZLIB=False
export BUILD_BZIP2=0
unset DESTDIR

sh Configure -des                                          \
             -D prefix=/usr                                \
             -D vendorprefix=/usr                          \
             -D privlib=/usr/lib/perl5/5.40/core_perl      \
             -D archlib=/usr/lib/perl5/5.40/core_perl      \
             -D sitelib=/usr/lib/perl5/5.40/site_perl      \
             -D sitearch=/usr/lib/perl5/5.40/site_perl     \
             -D vendorlib=/usr/lib/perl5/5.40/vendor_perl  \
             -D vendorarch=/usr/lib/perl5/5.40/vendor_perl \
             -D man1dir=/usr/share/man/man1                \
             -D man3dir=/usr/share/man/man3                \
             -D pager="/usr/bin/less -isR"                 \
             -D useshrplib                                 \
             -D usethreads

make
make install
unset BUILD_ZLIB BUILD_BZIP2

EOF
)

RDIR=$(cat <<EOF

echo not implemented

EOF
)

UDIR=$(cat <<EOF
EOF
)
