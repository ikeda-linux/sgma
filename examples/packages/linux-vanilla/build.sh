#!/usr/bin/env bash

src="$(pwd)/src"
out="$(pwd)/out"
dir="$(pwd)/."
ver="5.15.6"

inf () {
    echo "==> \033[1;32m$1\033[0m"
}

get () {
    inf "Getting source..."
    wget https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-${ver}.tar.xz
    tar -xvf linux-${ver}.tar.xz -C ${src}/
    wget https://raw.githubusercontent.com/archlinux/svntogit-packages/131a17b2319e4d56f3d95a1a2fca183e86474225/trunk/config
    cp config ${src}/linux-${ver}/.config
}

build () {
    cd ${src}/linux-${ver}
    mkdir -p ${out}/overlay/boot
    inf "Building..."
    make -j$(nproc)
    cd ${dir}
    inf "Copying binary..."
    cp ${src}/linux-${ver}/arch/$(arch)/boot/bzImage ${out}/overlay/boot 
}

clean () {
    inf "Getting rid of build artifacts..."
    rm -rf config > /dev/null 2>&1
    rm -rf linux-*.tar.xz > /dev/null 2>&1
    rm -rf ${src}/* ${src}/.* > /dev/null 2>&1
}

permissions () {
    inf "Setting correct permissions..."
    chown -R root:root ${out}/
    chmod -R 777 ${out}/
}

main () {
    get
    build
    clean
    permissions
    exit 0
}

main
