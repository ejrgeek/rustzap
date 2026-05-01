#!/bin/bash
set -e

cd flatpak

# Build the Flatpak (local)
mkdir -p build/files
cp ../src-tauri/target/release/rustzap build/files/
cp ../src-tauri/icons/128x128.png build/files/icon.png
cp br.com.erlondnjr.rustzap.desktop build/files/
cp br.com.erlondnjr.rustzap.metainfo.xml build/files/

flatpak-builder --user --install --force-clean build-dir br.com.erlondnjr.rustzap.local.yml --repo=repo
flatpak build-bundle repo rustzap.flatpak br.com.erlondnjr.rustzap