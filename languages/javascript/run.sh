#!/bin/sh
set -e

printf %s "$1" | node -p || true
