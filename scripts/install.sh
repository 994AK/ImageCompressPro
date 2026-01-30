#!/bin/bash
# Install ImageCompressTool from source
# This script ensures the rust toolchain is present and installs the binary

echo "Checking for Rust toolchain..."
if ! command -v cargo &> /dev/null; then
    echo "Cargo not found. Please install Rust: https://rustup.rs/"
    exit 1
fi

echo "Installing image-compress-tool from GitHub..."
# Install directly from the main repository where the Rust crate lives
# Note: This assumes the rust crate is at the root or a known path in the repo.
# Since we just moved the source code OUT of the skill package to clean it up,
# we need to decide:
# 1. Is the source code still in the GitHub repo? -> Yes, we are committing it to root.
# 2. Or do we point to a specific release tag? -> 'main' branch for now.

cargo install --git https://github.com/994AK/ImageCompressSkill.git image-compress-tool

echo "Installation complete! Verifying..."
image-compress-tool --help
