#!/usr/bin/env bash

src="$(pwd)/src"
out="$(pwd)/out"

inf () {
    echo "==> \033[1;32m$1\033[0m"
}

get () {
    inf "Getting source..."
    git clone xyz ${src}
}

make () {
    cd src/
    mkdir -p ${out}/overlay/foo
    inf "Building..."
    ./configure --prefix={out}/overlay
    ./make
    ./make install
}

permissions () {
    inf "Setting correct permissions..."
    chown -R root:root ${out}
    chmod -R 755 ${out}
}

clean () {
    inf "Getting rid of build artifacts..."
    rm -rf ${src}/* ${src}/.* > /dev/null 2>&1
}

main () {
    get
    make
    permissions
    clean
    exit 0
}

main