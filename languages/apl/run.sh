#!/bin/sh
set -e

printf %s "$1" > program.apl
apl --OFF -s -f program.apl || true
