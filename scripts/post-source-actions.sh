#!/bin/bash
set -e

YOCTO_BUILD_CONF_DIR="/mnt/lacie/yocto-raspberry/build/conf"

cp "/workdir/conf/local.conf" "$YOCTO_BUILD_CONF_DIR/local.conf"
cp "/workdir/conf/bblayers.conf" "$YOCTO_BUILD_CONF_DIR/bblayers.conf"
