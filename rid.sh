#!/bin/bash

. "$RIDENV" || die 'Failed to source $RIDENV'

"$RIDHOME"/target/release/rid "$@"
