#!/usr/bin/env bash

pkgname="foo"

postinstall_script () {
    rm -r "/opt/$pkgname"
}

main () {
    postinstall_script
    exit 0
}

main