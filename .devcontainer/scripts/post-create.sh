#!/bin/bash

# setup the git identities
git config --global user.name "${GIT_AUTHOR_NAME}"
git config --global user.email "${GIT_AUTHOR_EMAIL}"

# to install cargo binaries
cargo install cargo-binstall
# install cargo test runner and file watcher
cargo binstall cargo-watch cargo-nextes cargo-expand --secure --no-confirm
