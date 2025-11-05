#!/usr/bin/bash
BUILDDIR=builddir
if [ "$1" = "clean" ] | [ ! -d "$BUILDDIR"  ]; then
    echo "Rebuilding"
    meson setup builddir --prefix "$(pwd)/$BUILDDIR" --buildtype=debug --reconfigure -Dprofile=development --wipe
fi

meson compile -C $BUILDDIR
meson install -C $BUILDDIR -q
[ $? -eq 0 ]  || exit 1
# Install settings schema for current user if not already installed
SCHEMA=org.mydomain.MyRustApp.Devel.gschema.xml
if [ ! -f ~/.local/share/glib-2.0/schemas/$SCHEMA ]; then
	echo "Installing settings schema for current user.."
	mkdir -p ~/.local/share/glib-2.0/schemas
	cp "builddir/data/$SCHEMA" ~/.local/share/glib-2.0/schemas/.
	glib-compile-schemas ~/.local/share/glib-2.0/schemas/
fi
echo "Running application"
$BUILDDIR/bin/my_rust_app
