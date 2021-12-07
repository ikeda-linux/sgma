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

clean () {
    inf "Getting rid of build artifacts..."
    rm -rf ${src}/* ${src}/.* > /dev/null 2>&1
}

main () {
    get
    make
    clean
}

main
echo
exit 0"#;

pub static PRE_SH: &'static str =
r#"#!/usr/bin/env bash

pkgname="foo"

mkdir -p "/opt/$pkgname""#;

pub static POST_SH: &'static str =
r#"#!/usr/bin/env bash

pkgname="foo"

rm -r "/opt/$pkgname""#;

pub static HOOK_SH: &'static str =
r#"#!/usr/bin/env bash

hooknamne="wasting time"

echo "Doing something! (${hookname})" > /dev/null"#;