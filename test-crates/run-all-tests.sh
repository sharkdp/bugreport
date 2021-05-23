#!/usr/bin/env bash

set -o errexit -o nounset -o pipefail

script_path="$(pwd)/$(dirname ${BASH_SOURCE})"

test_crates="
bugreport-client-with-git-hash-feature
bugreport-client-without-git-hash-feature
"

for test_crate in ${test_crates}; do
    cd ${script_path}/${test_crate}

    # Test with a git repo (our own)
    unset GIT_DIR
    cargo clean -p ${test_crate}
    cargo test with_git_repo

    # Test without a git repo
    export GIT_DIR=this-is-not-a-git-repo
    cargo clean -p ${test_crate}
    cargo test without_git_repo
done
