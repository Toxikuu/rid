#!/bin/bash

METAFILE="$2"

function pre {
  bv=$(compgen -v)
  . /code/rid/meta/"$METAFILE"
  av=$(compgen -v)
}

function cleanup {
  for var in $after_vars; do
    if ! echo "$before_vars" | grep -q "^$var$"; then
      unset "$var"
    fi
  done
  rm -rf /tmp/rid/building/$NAME-$VERS
}

function ins {
  check_perms
  pushd /tmp/rid/building/$NAME-$VERS > /dev/null

  echo "Installing $NAME-$VERS..."
  eval "$IDIR"
}

function rem {
  check_perms

  echo "Removing $NAME-$VERS..."
  eval "$RDIR"
}

function check_perms {
  if [ "$EUID" -ne 0 ]; then
    echo "Insufficient permissions."
    exit 1
  fi
}

pre
case $1 in
  i)
    ins
    popd
    cleanup
    ;;
  r)
    rem
    cleanup
    ;;
  u)
    if [ -z "$UDIR" ]; then
      echo "Updating $NAME (default)..."
      rem
      ins
    else
      echo "Updating $NAME (custom)..."
      eval "$UDIR"
    fi
    cleanup
    ;;
  s)
    echo "Status of $NAME-$VERS: $STAT"
    cleanup
    ;;
  v)
    echo "NAME: $NAME"
    echo "VERS: $VERS"
    echo "LINK: $LINK"
    echo "STAT: $STAT"
    echo "DEPS: $DEPS"
    echo ""
    echo "IDIR: $IDIR"
    echo "RDIR: $RDIR"
    echo "UDIR: $UDIR"
    ;;
  *)
    echo "Invalid action: $2"
    echo "Valid actions:"
    echo "i - install"
    echo "r - remove"
    echo "u - update"
    echo "s - status"
    echo "v - variables"
    exit 1
    ;;
esac
