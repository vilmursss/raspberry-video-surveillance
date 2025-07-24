#!/bin/bash
set -e

source $VOLUME_WORKDIR/conf/build.conf

# Generate the bitbake-env.conf file from the build.conf file.
$VOLUME_WORKDIR/scripts/generate-bitbake-conf.sh "$VOLUME_WORKDIR/conf/build.conf" "$YOCTO_BUILD_DIR/conf/bitbake-env.conf"

cp "$VOLUME_WORKDIR/conf/local.conf" "$YOCTO_BUILD_DIR/conf/local.conf"
cp "$VOLUME_WORKDIR/conf/bblayers.conf" "$YOCTO_BUILD_DIR/conf/bblayers.conf"

# Copy the project layer to the yocto workdir. Required by BitBake in bblayers.conf.
mkdir -p "$YOCTO_WORKDIR/project-layers"
cp -r "$VOLUME_WORKDIR/meta-video-surveillance" "$YOCTO_WORKDIR/project-layers"