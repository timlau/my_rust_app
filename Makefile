NAME=$(shell grep -A1 "^project" meson.build | sed -n "2s/.*'\([^']*\)'.*/\1/p")
VERSION=$(shell sed -n "s/.*version: '\([0-9.]*\)'.*/\1/p" meson.build)
ARCHIVE_NAME = ${NAME}-${VERSION}.tar.gz
ARCHIVE_FILES = src/ data/ po/ Cargo.toml meson.build meson_options.txt README.md LICENSE my_rust_app.spec

all:
	@echo "Nothing to do, use a specific target"

list:
	@echo "NAME         : ${NAME}"
	@echo "VERSION      : ${VERSION}"
	@echo "ARCHIVE_NAME : ${ARCHIVE_NAME}"

archive:
	@echo "Creating archive.."
	@tar czf ${ARCHIVE_NAME} --transform "s,^,${NAME}-${VERSION}/," ${ARCHIVE_FILES}
	@echo "archive created: ${ARCHIVE_NAME}"

update-pot:
	@echo "Updating .pot file.."
	@xgettext --package-name=${NAME} --package-version=main --files-from=po/POTFILES.in --output=po/${NAME}.pot

PHONY: archive update-pot list

