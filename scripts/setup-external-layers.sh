#!/bin/bash
set -e

YOCTO_TAG="yocto-5.0.10"
EXTLAYERS_DIR="/mnt/lacie/yocto-raspberry/external-layers"

LAYER_REPOS=(
    "git://git.yoctoproject.org/poky.git poky $YOCTO_TAG"
)

mkdir -p "$EXTLAYERS_DIR"
cd "$EXTLAYERS_DIR"

for layer in "${LAYER_REPOS[@]}"
do
    set -- $layer
    REPO="$1"
    DIR="$2"
    TAG="${3:-}"
    if [ ! -d "$DIR" ]; then
        echo "Cloning $DIR at tag $TAG..."
        if [ -n "$TAG" ]; then
            git clone --depth 1 --branch "$TAG" "$REPO" "$DIR"
        else
            git clone --depth 1 "$REPO" "$DIR"
        fi
    else
        echo "$DIR already exists."
    fi
done
