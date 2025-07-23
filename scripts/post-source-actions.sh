#!/bin/bash
set -e

source $VOLUME_WORKDIR/conf/build.conf

cp "$VOLUME_WORKDIR/conf/local.conf" "$YOCTO_BUILD_DIR/conf/local.conf"
cp "$VOLUME_WORKDIR/conf/bblayers.conf" "$YOCTO_BUILD_DIR/conf/bblayers.conf"

# Copy the project layer to the yocto workdir. Required by BitBake in bblayers.conf.
mkdir -p "$YOCTO_WORKDIR/project-layers"
cp -r "$VOLUME_WORKDIR/meta-video-surveillance" "$YOCTO_WORKDIR/project-layers"