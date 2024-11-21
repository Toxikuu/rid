#!/bin/bash

. /code/rid/env || die "Failed to source dev rid's env"

/code/rid/target/release/rid "$@"
