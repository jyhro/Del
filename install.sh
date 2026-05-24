#!/usr/bin/env sh
set -eu

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: Rust toolchain not found."
    echo "Install it with:  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "Installing del..."
cargo install --path . --locked

echo ""
echo "Done! Binary installed to: ~/.cargo/bin/del"
if ! echo "$PATH" | grep -q "$HOME/.cargo/bin"; then
    echo "Add ~/.cargo/bin to your PATH:  export PATH=\"\$HOME/.cargo/bin:\$PATH\""
fi
