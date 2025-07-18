#!/bin/bash
set -e

YOCTO_WORKDIR="/mnt/lacie/yocto-raspberry"
BUILD_DIR="${YOCTO_WORKDIR}/build"
EXTERNAL_LAYERS="${YOCTO_WORKDIR}/external-layers"
POKY_PATH="${EXTERNAL_LAYERS}/poky"

mkdir -p "$BUILD_DIR" "$EXTERNAL_LAYERS"

PROJECT_ROOT=$(git rev-parse --show-toplevel)

docker build -f "$PROJECT_ROOT/docker/Dockerfile" -t yocto-builder "$PROJECT_ROOT/docker"

docker run -it --rm \
  --volume "$PROJECT_ROOT":/workdir:ro \
  --volume "$YOCTO_WORKDIR":/mnt/lacie/yocto-raspberry \
  --workdir /workdir \
  --name yocto-dev \
  yocto-builder \
  bash -c "bash /workdir/scripts/setup-external-layers.sh && \
           source $EXTERNAL_LAYERS/poky/oe-init-build-env $BUILD_DIR && \
           exec bash"
