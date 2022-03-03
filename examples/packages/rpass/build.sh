#!/usr/bin/env bash

src="$(pwd)/src"
out="$(pwd)/out"

inf () {
    echo "==> \033[1;32m$1\033[0m"
}

get () {
    inf "Getting source..."
    git clone https://git.tar.black/michal/passman ${src}
}

build () {
    cd ${src}
    mkdir -p ${out}/overlay/bin
    inf "Building..."
    cargo build --release --package rpass
    inf "Copying binary..."
    mkdir -p ${out}/bin
    cp target/release/rpass ${out}/overlay/bin
}

clean () {
    inf "Getting rid of build artifacts..."
    rm -rf ${src}/* ${src}/.* > /dev/null 2>&1
}

permissions () {
    inf "Setting correct permissions..."
    chown -R root:root ${out}
    chmod -R 755 ${out}
}

main () {
    get
    build
    clean
    permissions
    exit 0
}

main
