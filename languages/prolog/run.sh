#!/bin/sh
set -e

printf %s "$1" > program.pl
swipl --quiet program.pl || true
