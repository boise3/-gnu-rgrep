#!/bin/zsh

set -e

echo "Building rgrep..."
cargo build --release

INSTALL_DIR="$HOME/.local/bin"

mkdir -p "$INSTALL_DIR"
cp target/release/rgrep "$INSTALL_DIR/rgrep"
chmod +x "$INSTALL_DIR/rgrep"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "exporting $INSTALL_DIR to PATH..."

    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc"
    export PATH="$HOME/.local/bin:$PATH"
fi

echo ""
echo "rgrep installed!"
echo "Try:"
echo "  rgrep uwu grep.txt -c"