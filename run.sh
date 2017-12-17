#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

if [ $# -ne 0 ]
then
    TEST="_test"
else
	TEST=""
fi

YEAR=$(date +%Y)
DAY=$(date +%d)
echo cargo run ${YEAR} ${DAY} ./inputs/${YEAR}_${DAY}${TEST}.txt
cargo run ${YEAR} ${DAY} ./inputs/${YEAR}_${DAY}${TEST}.txt
