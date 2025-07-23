#!/bin/bash
set -e

source $(git rev-parse --show-toplevel)/conf/build.conf

if [ ! -d $IMAGE_DECOMPRESSED_DIR ]; then
    echo "$IMAGE_DECOMPRESSED_DIR does not exist. Please create directory first."
    exit 1
fi

bzcat $YOCTO_BUILD_DIR/tmp/deploy/images/raspberrypi3/core-image-minimal-raspberrypi3.rootfs.wic.bz2 > $IMAGE_DECOMPRESSED_DIR/core-image-minimal-raspberrypi3.rootfs.wic

echo "Image decompressed to $IMAGE_DECOMPRESSED_DIR/core-image-minimal-raspberrypi3.rootfs.wic"