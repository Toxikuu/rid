#!/bin/bash

. "$RIDHOME"/rid/env || die "Failed to source dev rid's env"

"$RIDHOME"/target/release/rid "$@"
