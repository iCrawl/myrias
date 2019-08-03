#!/bin/sh
set -e

printf %s "$1" > program.php
php program.php || true
