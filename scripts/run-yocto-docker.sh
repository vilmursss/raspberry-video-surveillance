#!/bin/bash
set -e

PROJECT_ROOT=$(git rev-parse --show-toplevel)

source $PROJECT_ROOT/conf/build.conf

EXTERNAL_LAYERS="$YOCTO_WORKDIR/external-layers"

mkdir -p "$YOCTO_WORKDIR" "$YOCTO_BUILD_DIR" "$EXTERNAL_LAYERS"

docker build -f "$PROJECT_ROOT/docker/Dockerfile" -t yocto-builder "$PROJECT_ROOT/docker"

docker run -it --rm \
  --volume "$PROJECT_ROOT":"$VOLUME_WORKDIR":ro \
  --volume "$YOCTO_WORKDIR":"$YOCTO_WORKDIR":rw \
  --name yocto-dev \
  yocto-builder \
  bash -c '
    export VOLUME_WORKDIR="'"$VOLUME_WORKDIR"'"
    bash '"$VOLUME_WORKDIR"'/scripts/setup-external-layers.sh &&
    source '"$EXTERNAL_LAYERS"'/poky/oe-init-build-env '"$YOCTO_BUILD_DIR"' &&
    bash '"$VOLUME_WORKDIR"'/scripts/post-source-actions.sh &&
    if [ $# -eq 0 ]; then
      exec bash
    else
      "$@"
    fi
  ' -- "$@"
