#!/bin/sh
set -e

printf %s "$1" > program.rkt
racket program.rkt || true
