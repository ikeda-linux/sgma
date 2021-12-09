pub static BUILD_SH: &'static str = 
r#"#!/usr/bin/env bash

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
"#;

pub static PRE_SH: &'static str =
r#"#!/usr/bin/env bash

pkgname="foo"

preinstall_script () {
    mkdir -p "/opt/$pkgname"
}

main () {
    preinstall_script
    exit 0
}

main
"#;

pub static POST_SH: &'static str =
r#"#!/usr/bin/env bash

pkgname="foo"

postinstall_script () {
    rm -r "/opt/$pkgname"
}

main () {
    postinstall_script
    exit 0
}

main
"#;

pub static HOOK_SH: &'static str =
r#"#!/usr/bin/env bash

hookname="wasting time"

hook () {
    echo "Doing something! (${hookname})" > /dev/null
}

main () {
    hook
    exit 0
}

main
"#;