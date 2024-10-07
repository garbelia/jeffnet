#!/bin/bash

echo "Welcome to JEFFNET v0.1.2"
echo "This setup file is currently incomplete, but should hopefully do everything right"
echo "Before proceeding, please ensure all of your nations have been listed in nations.txt inside the src-str directory, 1 nation per line."
read -p "Run specialism script? This will sort all of the nations in nations.txt into their N-Day specialism. It is advised to only run this script once. [y/n] " conf
if [ "$conf" == "y" ]; then
    cd src-str
    cargo run
    cd ..
fi
ln src-str/SORTEDnations.txt src-cat/nations.txt
echo "Running sheet generator..."
cd src-cat
cargo run
cd ..
echo "Sheet generated at /src-cat/sheet.html. Have fun!"