#!/bin/bash

if [ "$(id -u)" -ne 0 ]; then
  echo "This script must be run as root. Please use sudo."
  exit 1
fi

echo "Installing the binary..."
cd peo-support
cargo build
cd ..
cargo build
sudo cp -r target/debug/peo /usr/local/bin/
sudo cp -r peo-support/target/debug/peo-support /usr/local/bin/
mkdir ~/.local/share/peo/
cp -r peo.desktop ~/.local/share/peo/
echo "Installation completed successfully!"
