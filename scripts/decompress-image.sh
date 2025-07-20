#!/bin/bash
set -e

if [ ! -d /mnt/tmp ]; then
    echo "/mnt/tmp does not exist. Tmp needs to be mounted first."
    exit 1
fi

bzcat /mnt/lacie/yocto-raspberry/build/tmp/deploy/images/raspberrypi3/core-image-minimal-raspberrypi3.rootfs.wic.bz2 > /mnt/tmp/core-image-minimal-raspberrypi3.rootfs.wic

echo "Image decompressed to /mnt/tmp/core-image-minimal-raspberrypi3.rootfs.wic"