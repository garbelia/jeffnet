#!/bin/bash

echo "Welcome to JEFFNET v0.1.3"
echo "This setup file is currently incomplete, but should hopefully do everything right"
echo "Before proceeding, please ensure all of your nations have been listed in nations.txt inside this directory, 1 nation per line."
rm src-str/nations.txt
ln nations.txt src-str/nations.txt
read -p "Run specialism script? This will sort all of the nations in nations.txt into their N-Day specialism. It is advised to only run this script once. [y/n] " conf
if [ "$conf" == "y" ]; then
    cd src-str
    cargo run
    cd ..
    rm src-cat/nations.txt
    ln src-str/SORTEDnations.txt src-cat/nations.txt
    echo "Nation sorting complete. Please run this setup again and select no when prompted."
else
    echo "Running sheet generator..."
    cd src-cat
    cargo run
    cd ..
    echo "Sheet generated at /src-cat/jnday_sheet.html. Have fun!"
fi