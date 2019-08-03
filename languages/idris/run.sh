#!/bin/sh
set -e

printf %s "$1" > Main.idr
idris --execute ./Main.idr || true
