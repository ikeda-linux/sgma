#!/usr/bin/env bash

pkgname="foo"

preinstall_script () {
    mkdir -p "/opt/$pkgname"
}

main () {
    preinstall_script
    exit 0
}

main