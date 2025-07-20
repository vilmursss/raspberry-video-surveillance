#!/bin/bash
set -e

CURRENT_DIR=$(pwd)
PROJECT_ROOT=$(git rev-parse --show-toplevel)

# Find and process every Cargo.toml not inside a target folder
find "$PROJECT_ROOT" -name Cargo.toml -not -path "*/target/*" | while read -r toml; do
    dir=$(dirname "$toml")
    echo "==========================="
    echo "Running tests in $dir"
    cd "$dir"
    if cargo test; then
        echo "Tests passed, running cargo clean..."
        cargo clean
    else
        echo "Tests FAILED in $dir"
        cargo clean
        break
    fi
    echo "---------------------------"
done

cd "$CURRENT_DIR"