#!/bin/sh
set -e

printf %s "$1" > program.ml
ocamlopt -cclib --static -o program program.ml && ./program || true
