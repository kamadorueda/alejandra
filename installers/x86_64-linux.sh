#! /bin/sh

releases=https://github.com/kamadorueda/alejandra/releases/download/
target=alejandra-x86_64-unknown-linux-musl
version=0.1.0

curl -o alejandra -L "${releases}/${version}/${target}"

chmod +x alejandra
