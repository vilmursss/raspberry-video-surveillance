#!/bin/bash
set -e

source $VOLUME_WORKDIR/conf/build.conf

# Most recent (scarthgap) poky tag commit at 18.7.2025
POKY_TAG="ac257900c33754957b2696529682029d997a8f28"

# Most recent scarthgap commit at 18.7.2025
RASPBERRY_TAG="48452445d779b0f69bf1ead12b3850d50eb8920d"

# most recent scarthgap commit at 18.7.2025
OE_TAG="e8fd97d86af86cdcc5a6eb3f301cbaf6a2084943"

EXTLAYERS_DIR="$YOCTO_WORKDIR/external-layers"

LAYER_REPOS=(
    "https://git.yoctoproject.org/git/poky.git poky $POKY_TAG"
    "https://git.yoctoproject.org/meta-raspberrypi meta-raspberrypi $RASPBERRY_TAG"
    "https://github.com/openembedded/meta-openembedded.git meta-openembedded $OE_TAG"
)

mkdir -p "$EXTLAYERS_DIR"
cd "$EXTLAYERS_DIR"

for layer in "${LAYER_REPOS[@]}"
do
    set -- $layer
    REPO="$1"
    DIR="$2"
    REF="$3"

    if [ ! -d "$DIR" ]; then
        echo "Cloning $DIR at $REF..."
        git clone "$REPO" "$DIR"
        cd "$DIR"
        git checkout "$REF"
        cd ..
    else
        echo "$DIR already exists. Checking reference..."
        cd "$DIR"
        CURRENT_HASH=$(git rev-parse HEAD)
        TARGET_HASH=$(git rev-parse "$REF" 2>/dev/null || true)
        # If REF is a commit, TARGET_HASH is just REF
        [[ -z "$TARGET_HASH" ]] && TARGET_HASH="$REF"
        if [ "$CURRENT_HASH" != "$TARGET_HASH" ]; then
            echo "$DIR is at $CURRENT_HASH, switching to $REF..."
            git fetch --all
            git checkout "$REF"
        else
            echo "$DIR already at $REF ($CURRENT_HASH)"
        fi
        cd ..
    fi
done
