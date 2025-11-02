APPNAME=$(shell grep -A1 "^project" meson.build | sed -n "2s/.*'\([^']*\)'.*/\1/p")
VERSION=$(shell sed -n "s/.*version: '\([0-9.]*\)'.*/\1/p" meson.build)
ARCHIVE_NAME = ${APPNAME}-${VERSION}.tar.xz
ARCHIVE_FILES = src/ data/ po/ build-aux/ Cargo.toml meson.build meson_options.txt README.md LICENSE ${APPNAME}.spec Makefile

NAME = $(shell git config --get user.name)
EMAIL = $(shell git config --get user.email)
PACKAGER = $(NAME) <$(EMAIL)>
CURDIR = ${shell pwd}
BUILDDIR= $(CURDIR)/rpmbuild
RPMBUILD_OPTS = -D '_topdir $(BUILDDIR)' -D 'packager $(PACKAGER)'

all:
	@echo "Nothing to do, use a specific target"

list:
	@echo "APPNAME          : ${APPNAME}"
	@echo "VERSION          : ${VERSION}"
	@echo "ARCHIVE_NAME     : ${ARCHIVE_NAME}"
	@echo "ARCHIVE_FILES    : ${ARCHIVE_FILES}"
	@echo "BUILDDIR         : ${BUILDDIR}"
	@echo "RPMBUILD_OPTS    : ${RPMBUILD_OPTS}"

archive:
	@echo "Creating archive.."
	@rm -rf ${ARCHIVE_NAME}
	@tar cJf ${ARCHIVE_NAME} --transform "s,^,${APPNAME}-${VERSION}/," ${ARCHIVE_FILES}
	@mkdir -p ${BUILDDIR}/SOURCES
	@cp ${ARCHIVE_NAME} ${BUILDDIR}/SOURCES
	@rm -rf ${ARCHIVE_NAME}
	@echo "The archive is in ${ARCHIVE_NAME}"

rpm:
	@$(MAKE) archive
	@echo "Building RPM.."
	@-rpmbuild $(RPMBUILD_OPTS) -ba ${APPNAME}.spec


flatpak:
	@echo "Building Flatpak.."
	flatpak-builder --user flatpak_app build-aux/org.mydomain.MyRustApp.Devel.json --force-clean

flatpak-deps:
	flatpak install --user org.gnome.Sdk//48 org.gnome.Platform//48  org.freedesktop.Sdk.Extension.rust-stable//24.08 org.freedesktop.Sdk.Extension.llvm18//24.08

flatpak-run:
	flatpak-builder --run flatpak_app build-aux/org.mydomain.MyRustApp.Devel.json my_rust_app

update-pot:
	@echo "Updating .pot file.."
	@xgettext --package-APPNAME=${APPNAME} --package-version=main --files-from=po/POTFILES.in --output=po/${APPNAME}.pot

PHONY: archive update-pot list

