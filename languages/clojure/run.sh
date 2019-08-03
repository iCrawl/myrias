#!/bin/sh
set -e

printf %s "$1" > program.clj
clojure program.clj || true
