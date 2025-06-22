# actlog

⚙️ A Rust-based CLI tool to inspect running processes, open ports, and kill misbehaving PIDs or ports from the terminal.

## Features

- List running processes
- List open ports
- Kill processes by PID
- Kill processes by port number
- Cross-platform support (macOS and Linux)

## Installation

### Prerequisites

- Rust and Cargo must be installed on your system
- If you don't have Rust installed, visit [https://rustup.rs/](https://rustup.rs/) or run:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Quick Install

1. Clone or download this repository
2. Navigate to the project directory
3. Run the install script:

```bash
./install.sh
```

The script will:

- Check if Rust is installed
- Build the project in release mode
- Install the binary to `~/.local/bin`
- Add the installation directory to your PATH
- Verify the installation

### Manual Installation

If you prefer to install manually:

```bash
# Build the project
cargo build --release

# Create installation directory
mkdir -p ~/.local/bin

# Copy the binary
cp target/release/actlog ~/.local/bin/

# Make it executable
chmod +x ~/.local/bin/actlog

# Add to PATH (add this to your ~/.zshrc or ~/.bashrc)
export PATH="$HOME/.local/bin:$PATH"
```

## Usage

After installation, you can use `actlog` from anywhere in your terminal:

```bash
# Show help
actlog --help

# Show version
actlog --version

# Show list command help
actlog list --help
```

## Uninstallation

To uninstall actlog:

```bash
./install.sh --uninstall
```

This will:

- Remove the binary from `~/.local/bin`
- Remove the PATH configuration from your shell config files

## Development

### Building for Development

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running in Development Mode

```bash
cargo run -- --help
```

## License

MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Author

Morshedul Munna <morshedulmunna1@gmail.com>
