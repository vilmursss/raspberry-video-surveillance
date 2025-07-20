#!/bin/bash
set -e

PROJECT_ROOT=$(git rev-parse --show-toplevel)

$PROJECT_ROOT/scripts/run-yocto-docker.sh bitbake core-image-minimal

$PROJECT_ROOT/scripts/decompress-image.sh