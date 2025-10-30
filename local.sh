#!/usr/bin/bash
BUILDDIR=builddir
if [ "$1" = "clean" ] | [ ! -d "$BUILDDIR"  ]; then
    echo "Rebuilding"
    meson setup builddir --prefix "$(pwd)/$BUILDDIR" --buildtype=debug --reconfigure -Dprofile=development --wipe
fi

meson compile -C $BUILDDIR
meson install -C $BUILDDIR -q

echo "Running application"
$BUILDDIR/bin/my_rust_app