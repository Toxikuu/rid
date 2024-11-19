#!/bin/bash

. "$RIDHOME"/env || die "Failed to source rid's env"

"$RIDHOME"/target/release/rid "$@"
