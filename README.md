# bech32m

A fast, cross-platform CLI utility for converting data to and from [Bech32m](https://bips.dev/350/) encoded strings.

## Features

- **Decode** Bech32m strings to hexadecimal
- **Encode** data to Bech32m with a custom human-readable prefix
- **Auto-detect** input format: Bech32m, Base16 (hex), or Base58
- **Convert** between Bech32m strings with different prefixes
- **Cross-platform**: Linux, macOS, and Windows binaries available

## Installation

### From Releases

Download the latest binary for your platform from the [Releases](../../releases) page.

### From Source

```bash
cargo install --git https://github.com/yourusername/bech32m
```

Or build locally:

```bash
git clone https://github.com/yourusername/bech32m
cd bech32m
cargo build --release
# Binary will be at target/release/bech32m
```

## Usage

Data is read from standard input.

### Decode Bech32m to Hex

When no prefix is provided, the tool decodes a Bech32m string to hexadecimal:

```bash
$ echo "base16_1wpshgcgvnlscf" | bech32m
706174617465
```

### Encode to Bech32m

When a prefix is provided, the tool encodes the input to Bech32m:

```bash
# From hex (Base16)
$ echo "706174617465" | bech32m base16_
base16_1wpshgcgvnlscf

# From Base58
$ echo "Ae2tdPwUPEYy" | bech32m base58_
base58_1p58rejhd9592uusa8rvpy

# From another Bech32m (prefix conversion)
$ echo "old_prefix1wpshgcge5yvsv" | bech32m new_prefix
new_prefix1wpshgcg703stt
```

### Help

```bash
$ bech32m --help
Convert to and from bech32m strings. Data are read from standard input.

Usage: bech32m [PREFIX]

Arguments:
  [PREFIX]  An optional human-readable prefix (e.g. 'addr').
            When provided, the input text is decoded from various encoding
            formats and re-encoded to bech32m using the given prefix.
            When omitted, the input text is decoded from bech32m to base16.

Options:
  -h, --help     Print help
  -V, --version  Print version

Supported encoding formats: Base16, Bech32m & Base58.
```

## Examples

### Working with Bitcoin/Cryptocurrency Addresses

```bash
# Decode a shielded address to see its raw hex data
$ echo "mn_shield-addr_test1dfv46yhqklvgh4kzaw9p8dpezydetjeccssc7y2p32keeaqeuy4sxqqc60ndd9aahqlyyr7k8rhq5l2f7kc3y28759geed4clwzgdlg0ucgahz2x" | bech32m
6a595d12e0b7d88bd6c2eb8a13b439111b95cb38c4218f11418aad9cf419e12b030018d3e6d697bdb83e420fd638ee0a7d49f5b11228fea1519cb6b8fb8486fd0fe6

# Re-encode hex data with a new prefix
$ echo "6a595d12e0b7d88b" | bech32m myprefix
myprefix1dfv46yhqk8qncr4
```

### Roundtrip Verification

```bash
# Encode then decode to verify
$ echo "hello" | xxd -p | bech32m test
test1wejkxar9wg6t4jr

$ echo "test1wejkxar9wg6t4jr" | bech32m
68656c6c6f

$ echo "68656c6c6f" | xxd -r -p
hello
```

## Supported Input Formats

| Format | Description | Example |
|--------|-------------|---------|
| Bech32m | Bech32m encoded string | `bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4` |
| Base16 (Hex) | Hexadecimal string | `706174617465` |
| Base58 | Base58 encoded string | `Ae2tdPwUPEYy` |

The tool automatically detects the input format when encoding, trying formats in this order:
1. Bech32/Bech32m
2. Base16 (hex)
3. Base58

## Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with clippy lints
cargo clippy --all-targets --all-features -- -D warnings
```

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.

