#!/bin/sh
set -eu

# Get env
PREFIX="${PREFIX:-/usr/local}"

# Build the package
echo "Building package..."
cargo build --release

# Install the items
echo "Installing items..."
install -v -m 0755 "target/release/ezinstall" "$PREFIX/bin/ezinstall"