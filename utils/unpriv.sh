#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: $0 <command>"
  exit 1
fi

NON_ROOT_USER="t"

su -c "$*" $NON_ROOT_USER
