# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    export RUSTFLAGS="-D warnings"
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features
    cross check --target $TARGET --all-features
    cross check --target $TARGET --release --all-features

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
