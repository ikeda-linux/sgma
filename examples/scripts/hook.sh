#!/usr/bin/env bash

hookname="wasting time"

hook () {
    echo "Doing something! (${hookname})" > /dev/null
}

main () {
    hook
    exit 0
}

main