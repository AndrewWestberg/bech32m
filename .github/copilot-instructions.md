# GitHub Copilot Instructions for bech32m

## Project Overview

bech32m is a Rust CLI utility for converting data to and from bech32m encoded strings. It supports multiple input formats including Base16 (hex), Base58, and existing Bech32/Bech32m strings, making it useful for working with Bitcoin addresses and other bech32m-encoded data.

**Repository Type**: Rust CLI application
**Language**: Rust (edition 2021)
**Size**: Small project (single source file)
**Target Runtime**: Cross-platform (Linux, macOS, Windows)
**Main Dependencies**: bech32, bs58, hex, clap

## Build and Validation Commands

### Required Tools
- Rust toolchain (stable)
- Cargo (included with Rust)
- Components: rustfmt, clippy (for CI compliance)

### Build Commands (in order of typical usage)

1. **Quick validation** (fastest):
   ```bash
   cargo check
   ```

2. **Format check** (always run before commit):
   ```bash
   cargo fmt --all -- --check
   ```

3. **Linting** (always run before commit):
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```
   Note: The `-D warnings` flag treats warnings as errors (CI requirement).

4. **Development build**:
   ```bash
   cargo build --verbose
   ```

5. **Run tests**:
   ```bash
   cargo test --verbose
   ```

6. **Release build**:
   ```bash
   cargo build --release --verbose
   ```

7. **Run application**:
   ```bash
   # Decode bech32m to hex
   echo "test1wpshgct5v5hd5wlx" | cargo run

   # Encode hex to bech32m with prefix
   echo "706174617465" | cargo run -- myprefix
   ```

8. **Clean build artifacts**:
   ```bash
   cargo clean
   ```

### Validated Build Sequence
The recommended full validation sequence is:
1. `cargo fmt --all -- --check` (formatting)
2. `cargo clippy --all-targets --all-features -- -D warnings` (linting)
3. `cargo build --verbose` (dev build)
4. `cargo test --verbose` (tests)
5. `cargo build --release --verbose` (release build)

### Environment Setup
No special environment setup required beyond standard Rust installation. The project builds cleanly from a fresh clone.

## Project Layout and Architecture

### Directory Structure
```
/
├── .github/
│   └── workflows/          # CI/CD pipelines
│       ├── test.yml        # PR and push testing
│       └── release.yml     # Tagged release builds
├── .gitignore             # Standard Rust gitignore
├── Cargo.toml             # Project manifest
├── Cargo.lock             # Dependency lock file
├── LICENSE                # Apache 2.0 license
├── README.md              # Project documentation
├── src/
│   └── main.rs            # Entry point and all logic
└── target/                # Build artifacts (git-ignored)
```

### Key Dependencies
- **bech32**: Bech32/Bech32m encoding and decoding
- **bs58**: Base58 encoding and decoding
- **hex**: Hexadecimal encoding and decoding
- **clap**: Command-line argument parsing with derive macros

### Continuous Integration Requirements
The GitHub workflows enforce strict quality checks:
- **Conventional commits validation** with automatic detection of tag existence
- Code formatting with `cargo fmt --all -- --check`
- Linting with `cargo clippy --all-targets --all-features -- -D warnings`
- All tests must pass
- Both debug and release builds must succeed
- Release workflow creates binaries for multiple platforms (Linux, macOS, Windows)

**Conventional Commit Format Required:**
All commits must follow conventional commit format (e.g., `feat:`, `fix:`, `docs:`, `chore:`).

Examples of valid commits:
- `feat: add base32 input support`
- `fix: handle empty input gracefully`
- `docs: update README with usage examples`
- `chore: update dependencies`

### Application Features
- **Decode mode** (no prefix argument): Decodes bech32m string to hexadecimal
- **Encode mode** (with prefix argument): Encodes input to bech32m with the given human-readable prefix
- **Auto-detection**: Automatically detects input format (Bech32m, Base16/hex, Base58)
- **Prefix conversion**: Can convert between bech32m strings with different prefixes

### Critical Notes for Agents
1. **Always run formatting and linting checks** before submitting changes - the CI pipeline will fail without them
2. **Edition 2021**: This project uses Rust edition 2021
3. **CLI-focused**: Design with command-line interface as primary target
4. **Input via stdin**: Data is read from standard input, not command-line arguments

### Trust These Instructions
These instructions were created by comprehensively testing all build commands, exploring the complete codebase, and validating the CI/CD pipeline. The build and validation commands have been verified to work correctly in the current environment.
