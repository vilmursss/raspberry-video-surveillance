#!/bin/sh
set -e

DEVNAME="/dev/$1"
MOUNT_POINT="/mnt/usb"

[ -d "$MOUNT_POINT" ] || mkdir -p "$MOUNT_POINT"

mount "$DEVNAME" "$MOUNT_POINT" 2>&1

if [ $? -eq 0 ]; then
    echo "$(date) -- $DEVNAME mounted at $MOUNT_POINT" >> /tmp/usb-automount.log

    echo "$(date) -- Running wifi-connect..." >> /tmp/usb-automount.log
    /usr/bin/wifi-connect "$MOUNT_POINT"
    WIFI_RESULT=$?

    if [ $WIFI_RESULT -eq 0 ]; then
        echo "$(date) -- wifi-connect successful, starting web-server..." >> /tmp/usb-automount.log

        # Start web-server in the background
        /usr/bin/web-server "$MOUNT_POINT" &
        echo "$(date) -- web-server started" >> /tmp/usb-automount.log
        exit "$WIFI_RESULT"
    else
        echo "$(date) -- wifi-connect failed with code $WIFI_RESULT, not starting web-server" >> /tmp/usb-automount.log
        exit "$WIFI_RESULT"
    fi
else
    echo "$(date) -- Failed to mount $DEVNAME" >> /tmp/usb-automount.log
    exit 1
fi