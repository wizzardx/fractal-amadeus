#!/bin/bash
# Suggested repo path: test.sh
# Test script for running all Fractal Amadeus tests

set -e  # Exit immediately if a command exits with a non-zero status
set -x  # Print commands and their arguments as they are executed
poetry install
# Run repomix only if the command is available
if command -v repomix &> /dev/null; then
    repomix
    tree -h -L 2
else
    echo "Repomix not found, skipping..."
fi
git status
poetry run pytest --cov=fractal_amadeus --exitfirst --failed-first tests/

echo
echo "All tests succeeded successfully!"
echo
