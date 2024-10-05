#!/bin/bash

rm -rf /tmp/rid/building/"$1"/*
mv -Tvf /tmp/rid/extraction/* /tmp/rid/building/"$1"
