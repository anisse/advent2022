#!/bin/bash
#
set -euo pipefail
day=$1
day0=$(printf "%02d" "$1")

curl -sf --output "./src/inputs/$day0.txt" "https://adventofcode.com/2022/day/$day/input" --compressed -H 'User-Agent: Anisse AoC fetcher' -H 'Referer: https://adventofcode.com/2022/day/$day' -H "Cookie: $(cat cookie.txt)"
cp src/bin/template.rs src/bin/day$day0.rs
