#!/bin/sh
set -e

printf %s "$1" > program.sh
bash program.sh || true
