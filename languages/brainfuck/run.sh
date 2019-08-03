#!/bin/sh
set -e

printf %s "$1" | bf || true
