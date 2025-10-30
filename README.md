# GTK + Rust + Relm4 + Meson + Flatpak = <3

> This is a fork of [gtk-rust-template](https://gitlab.gnome.org/World/Rust/gtk-rust-template) that adapts the code for Relm4 while trying to change as little as possible.

A boilerplate template to get started with GTK, Rust, Meson, Flatpak made for GNOME. It can be adapted for other desktop environments like elementary.

<div align="center">

![Main window](data/resources/screenshots/screenshot1.png "Main window")
</div>

## What does it contains?

- A simple window with a headerbar
- Bunch of useful files that you SHOULD ship with your application on Linux:
  - Metainfo: describe your application for the different application stores out there;
  - Desktop: the application launcher;
  - Icons: This repo contains three icons, a normal, a nightly & monochromatic icon (symbolic) per the GNOME HIG, exported using [App Icon Preview](https://flathub.org/apps/details/org.gnome.design.AppIconPreview).
- Flatpak Manifest for nightly builds
- Dual installation support
- Uses Meson for building the application
- Bundles the UI files & the CSS using gresources
- A pre-commit hook to run rustfmt on your code
- Tests to validate your Metainfo, Schemas & Desktop files
- Gsettings to store the window state, more settings could be added
- i18n support


## Building the project

Make sure you have `flatpak` and `flatpak-builder` installed. Then run the commands below. Replace `<application_id>` with the value you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```shell
flatpak install --user org.gnome.Sdk//46 org.gnome.Platform//46  org.freedesktop.Sdk.Extension.rust-stable//24.08 org.freedesktop.Sdk.Extension.llvm16//24.08
flatpak-builder --user flatpak_app build-aux/org.mydomain.MyRustApp.Devel.json
```

## Running the project

Once the project is build, run the command below. Replace `<application_id>` and `<project_name>` with the values you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```shell
flatpak-builder --run flatpak_app build-aux/org.mydomain.MyRustApp.Devel.json my_rust_app
```

## Translations with Gettext

The template uses `gettext` as a framework for translations using [`gettext-rs`](https://github.com/gettext-rs/gettext-rs). The basic files for this can be found in the `po` folder.
While meson will take care of building the translations the extraction and translation itself has to be done manually.

### Extracting translatable strings

First of all you have to have `gettext` installed on your system. With that you then are able to use `xgettext` as following to extract the translatable strings:

```shell
xgettext --package-name=my_rust_app --package-version=main --files-from=po/POTFILES.in --output=po/my_rust_app.pot
```

Note that you might need to update the `po/POTFILES.in` file to reflect the files of your process. This describes where `xgettext` is going to search for strings to translate.

### Translating the translatable strings

To translate the strings you need to use po files. Tools like Poedit allow you to generate these from the `po/<project_name>.pot` file.
It also allows you to sync the `po/<project_name>.pot` when you rerun `xgettext`.

When adding a po file also make sure to add the language code to `po/LINGUAS`.

