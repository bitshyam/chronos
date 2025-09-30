# Chronos - Time Conversion CLI Tool

A fast and simple command-line tool for converting between ISO timestamps, epoch timestamps, and local time.

## Features

- 🕐 **Auto-detection**: Automatically detects whether input is an epoch timestamp or ISO timestamp
- 🌍 **Timezone aware**: Converts to your local timezone automatically
- 📅 **Flexible formats**: Supports multiple ISO timestamp formats (RFC3339, RFC2822, and common variations)
- ⚡ **Fast**: Built in Rust for maximum performance
- 🔧 **Simple**: Just pass the timestamp - no complex flags or subcommands needed

## Installation

### Pre-built Binaries (Recommended)

Download the latest release for your platform:

**Linux (x86_64)**
```bash
curl -L https://github.com/yourusername/chronos/releases/latest/download/chronos-linux-x86_64.tar.gz | tar xz
sudo mv chronos /usr/local/bin/
```

**Linux (ARM64)**
```bash
curl -L https://github.com/yourusername/chronos/releases/latest/download/chronos-linux-aarch64.tar.gz | tar xz
sudo mv chronos /usr/local/bin/
```

**macOS (Intel)**
```bash
curl -L https://github.com/yourusername/chronos/releases/latest/download/chronos-macos-x86_64.tar.gz | tar xz
sudo mv chronos /usr/local/bin/
```

**macOS (Apple Silicon)**
```bash
curl -L https://github.com/yourusername/chronos/releases/latest/download/chronos-macos-aarch64.tar.gz | tar xz
sudo mv chronos /usr/local/bin/
```

**Windows**
1. Download `chronos-windows-x86_64.zip` from the [releases page](https://github.com/yourusername/chronos/releases)
2. Extract `chronos.exe` 
3. Add the directory containing `chronos.exe` to your PATH

### Using Cargo (Build from Source)

If you have Rust installed:

```bash
cargo install --git https://github.com/yourusername/chronos
```

Or clone and build locally:

```bash
git clone https://github.com/yourusername/chronos
cd chronos
cargo build --release
sudo cp target/release/chronos /usr/local/bin/
```

## Usage

### Convert Epoch to ISO and Local Time

```bash
chronos 1702664832
```

Output:
```
Epoch: 1702664832
ISO: 2023-12-15T18:27:12+00:00
Local: 2023-12-15 23:57:12 +05:30
```

### Convert ISO to Epoch and Local Time

```bash
chronos "2025-12-01T10:20:01.899Z"
```

Output:
```
ISO: 2025-12-01T10:20:01.899Z
Epoch: 1764584401
Local: 2025-12-01 15:50:01 +05:30
```

### Get Current Time

```bash
chronos
```

Output:
```
Current time:
ISO: 2023-12-15T18:30:45.123456+00:00
Epoch: 1702665045
Local: 2023-12-15 23:00:45 +05:30
```

## Supported ISO Formats

- RFC3339: `2023-12-25T10:30:00Z`, `2023-12-25T10:30:00+05:30`
- RFC2822: `Mon, 25 Dec 2023 10:30:00 +0000`
- Common formats: `2023-12-25 10:30:00`, `2023-12-25T10:30:00`
- With microseconds: `2023-12-25T10:30:00.123456Z`

## Examples

```bash
# Various epoch formats
chronos 1703500200
chronos 1703500200000  # milliseconds (automatically detected)

# Various ISO formats
chronos "2023-12-25T10:30:00Z"
chronos "2023-12-25T10:30:00+05:30"
chronos "2023-12-25 10:30:00"
chronos "Mon, 25 Dec 2023 10:30:00 +0000"

# Current time
chronos
```

## Help

```bash
chronos --help
```

## Building

Requirements:
- Rust 1.70+ 

```bash
git clone https://github.com/yourusername/chronos
cd chronos
cargo build --release
```

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
