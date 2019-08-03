#!/bin/sh
set -e

printf %s "$1" > program.py
python program.py || true
