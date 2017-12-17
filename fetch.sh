#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

SESSION=$(cat session.txt)

YEAR=$(date +%Y)
DAY=$(date +%d)
echo "Fetching http://adventofcode.com/${YEAR}/day/${DAY}/input to inputs/${YEAR}_${DAY}.txt"
curl "http://adventofcode.com/${YEAR}/day/${DAY}/input" -H "Cookie: session=$SESSION" > inputs/${YEAR}_${DAY}.txt
git add inputs/2017_${DAY}.txt
git status
