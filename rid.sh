#!/bin/bash

set -e
. "$RIDENV"

"$RIDHOME"/target/release/rid "$@"
