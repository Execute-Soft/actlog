#!/bin/bash

# actlog CLI Tool Installer
# Supports macOS and Linux

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to detect OS
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    else
        echo "unknown"
    fi
}

# Function to get installation directory
get_install_dir() {
    local os=$(detect_os)
    if [[ "$os" == "macos" ]]; then
        echo "$HOME/.local/bin"
    else
        echo "$HOME/.local/bin"
    fi
}

# Function to check Rust installation
check_rust() {
    if ! command_exists rustc; then
        print_error "Rust is not installed. Please install Rust first:"
        echo "  Visit: https://rustup.rs/"
        echo "  Or run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    if ! command_exists cargo; then
        print_error "Cargo is not installed. Please install Rust with Cargo."
        exit 1
    fi
    
    print_success "Rust and Cargo are installed"
}

# Function to build the project
build_project() {
    print_status "Building actlog CLI tool..."
    
    if ! cargo build --release; then
        print_error "Failed to build the project"
        exit 1
    fi
    
    print_success "Project built successfully"
}

# Function to install the binary
install_binary() {
    local install_dir=$(get_install_dir)
    local binary_path="target/release/actlog"
    
    # Create installation directory if it doesn't exist
    mkdir -p "$install_dir"
    
    # Copy binary to installation directory
    print_status "Installing actlog to $install_dir..."
    
    if cp "$binary_path" "$install_dir/"; then
        print_success "Binary installed to $install_dir/actlog"
    else
        print_error "Failed to install binary"
        exit 1
    fi
    
    # Make it executable
    chmod +x "$install_dir/actlog"
    
    # Add to PATH if not already there
    local shell_rc=""
    if [[ -f "$HOME/.zshrc" ]]; then
        shell_rc="$HOME/.zshrc"
    elif [[ -f "$HOME/.bashrc" ]]; then
        shell_rc="$HOME/.bashrc"
    elif [[ -f "$HOME/.bash_profile" ]]; then
        shell_rc="$HOME/.bash_profile"
    fi
    
    if [[ -n "$shell_rc" ]]; then
        if ! grep -q "$install_dir" "$shell_rc"; then
            print_status "Adding $install_dir to PATH in $shell_rc"
            echo "" >> "$shell_rc"
            echo "# actlog CLI tool" >> "$shell_rc"
            echo "export PATH=\"$install_dir:\$PATH\"" >> "$shell_rc"
            print_success "PATH updated. Please restart your terminal or run: source $shell_rc"
        else
            print_success "PATH already configured"
        fi
    else
        print_warning "Could not find shell configuration file. Please manually add $install_dir to your PATH"
    fi
}

# Function to verify installation
verify_installation() {
    local install_dir=$(get_install_dir)
    
    if [[ -f "$install_dir/actlog" ]]; then
        print_success "Installation verified!"
        echo ""
        echo "actlog CLI tool has been installed successfully!"
        echo ""
        echo "Usage:"
        echo "  actlog --help                    # Show help"
        echo "  actlog --version                 # Show version"
        echo "  actlog list --help               # Show list command help"
        echo ""
        echo "If you can't run 'actlog' immediately, please restart your terminal"
        echo "or run: source ~/.zshrc (or ~/.bashrc)"
    else
        print_error "Installation verification failed"
        exit 1
    fi
}

# Function to uninstall
uninstall() {
    local install_dir=$(get_install_dir)
    
    print_status "Uninstalling actlog..."
    
    if [[ -f "$install_dir/actlog" ]]; then
        rm "$install_dir/actlog"
        print_success "actlog binary removed"
    else
        print_warning "actlog binary not found in $install_dir"
    fi
    
    # Remove from PATH in shell config files
    local shell_rcs=("$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.bash_profile")
    
    for shell_rc in "${shell_rcs[@]}"; do
        if [[ -f "$shell_rc" ]]; then
            if grep -q "actlog CLI tool" "$shell_rc"; then
                print_status "Removing actlog PATH configuration from $shell_rc"
                # Remove the actlog-related lines
                sed -i.bak '/# actlog CLI tool/,+1d' "$shell_rc"
                print_success "PATH configuration removed from $shell_rc"
            fi
        fi
    done
    
    print_success "Uninstallation completed"
}

# Main script
main() {
    echo "⚙️  actlog CLI Tool Installer"
    echo "================================"
    echo ""
    
    # Check if uninstall flag is provided
    if [[ "$1" == "--uninstall" ]]; then
        uninstall
        exit 0
    fi
    
    # Detect OS
    local os=$(detect_os)
    print_status "Detected OS: $os"
    
    # Check Rust installation
    check_rust
    
    # Build the project
    build_project
    
    # Install the binary
    install_binary
    
    # Verify installation
    verify_installation
}

# Run main function with all arguments
main "$@" 