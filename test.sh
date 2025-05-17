#!/bin/bash

# Common logic
set -e
set -x
repomix
tree -h -L 2
git status
tests/e2e_smoke_test.py

echo
echo "All the tests stucceed successfully!"
echo
