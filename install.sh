#!/bin/bash
# meant to be run from rid with `rid -b`
# will build the binary with cargo and rustc if nonexistent

set -e

[ "$EUID" -eq 0     ]   ||  { echo "Insufficient permissions" >&2   ; exit 1        ; }
[ -z "$RIDSOURCES"  ]   &&  { echo '$RIDSOURCES not set'            ; exit 1        ; }
[ -z "$RIDMETA"     ]   &&  { echo '$RIDMETA not set'               ; exit 1        ; }
[ -z "$RIDBIN"      ]   &&  { echo '$RIDBIN not set'                ; exit 1        ; }
[ -e /usr/bin/rid   ]   ||  { echo "Missing rid" >&2                ; BUILDRID=1    ; }

echo        "Extracting rid-root tarball..."
tar xf      "$RIDSOURCES"/rid-root.tar.xz -C "$RIDHOME"

[ -z "$BUILDRID" ]      ||  {
echo        "Building rid..."
pushd       "$RIDHOME" >/dev/null
bash        "$RIDHOME"/release.sh
ln -sfv     "$RIDHOME"/target/release/rid /usr/bin/rid
popd        >/dev/null
}

echo        "Extracting rid-meta tarball..."
tar xf      "$RIDSOURCES"/master.tar.gz -C "$RIDMETA"

echo        "Adjusting permissions..."
chmod 755   "$RIDBIN"/*

echo       "Done!"
