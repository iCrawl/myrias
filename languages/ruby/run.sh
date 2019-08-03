#!/bin/sh
set -e

printf %s "$1" > program.rb
ruby program.rb || true
