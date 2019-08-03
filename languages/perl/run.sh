#!/bin/sh
set -e

printf %s "$1" > program.pl
perl program.pl || true
