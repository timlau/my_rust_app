## Install flatpak build dependencies
```
flatpak install --user org.gnome.Sdk//47 org.gnome.Platform//47  org.freedesktop.Sdk.Extension.rust-stable//24.08 org.freedesktop.Sdk.Extension.llvm18//24.08

```

## build the flatpak
```
app_id=org.mydomain.MyRustApp
rm -rf .flatpak-builder/ flatpak_app/
flatpak-builder --user flatpak_app build-aux/$app_id.Devel.json
```

## create translation .pot file
```
app_name=my_rust_app
xgettext --package-name=$app_name --package-version=main --files-from=po/POTFILES.in --output=po/$app_name.pot
```

## Run the flatpak
```
app_id=org.mydomain.MyRustApp
flatpak-builder --run flatpak_app build-aux/$app_id.Devel.json my_rust_app
```