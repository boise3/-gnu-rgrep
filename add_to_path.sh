#!/bin/sh

set -e

echo "Building rgrep..."
cargo build --release

INSTALL_DIR="$HOME/.local/bin"

echo "Installing to $INSTALL_DIR..."

mkdir -p "$INSTALL_DIR"
cp target/release/rgrep "$INSTALL_DIR/rgrep"
chmod +x "$INSTALL_DIR/rgrep"

# !!!!!! Add ~/.local/bin to PATH if it isn't already there !!!!!!
case ":$PATH:" in
    *":$INSTALL_DIR:"*)
        echo "$INSTALL_DIR is already in PATH."
        ;;
    *)
        echo "Adding $INSTALL_DIR to PATH..."

        SHELL_RC="$HOME/.profile"

        if [ -n "$ZSH_VERSION" ] || [ "${SHELL##*/}" = "zsh" ]; then
            SHELL_RC="$HOME/.zshrc"
        fi

        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
        export PATH="$HOME/.local/bin:$PATH"

        echo "Added to $SHELL_RC"
        ;;
esac

echo ""
echo "rgrep installed successfully!"
echo "Try:"
echo "  rgrep uwu grep.txt -c"