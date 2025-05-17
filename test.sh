#!/bin/bash

set -e
set -x
poetry install
repomix
tree -h -L 2
git status
poetry run pytest --cov=fractal_amadeus tests/

# This one is significant, so we run it again at the end:
tests/functional_tests/test_basic_usage_scenario.py

echo
echo "All the tests stucceed successfully!"
echo
