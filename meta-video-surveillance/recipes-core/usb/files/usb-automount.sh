#!/bin/sh
set -e

DEVNAME="/dev/$1"
MOUNT_POINT="/mnt/usb"

[ -d "$MOUNT_POINT" ] || mkdir -p "$MOUNT_POINT"

mount "$DEVNAME" "$MOUNT_POINT" 2>&1
    
if [ $? -eq 0 ]; then
    echo "$(date) -- $DEVNAME mounted at $MOUNT_POINT" >> /tmp/usb-automount.log
    /usr/bin/wifi-connect "$MOUNT_POINT"
else
    echo "$(date) -- Failed to mount $DEVNAME" >> /tmp/usb-automount.log
fi