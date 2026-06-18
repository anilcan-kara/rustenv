# rustenv

A fast, secure environment variable and secret management tool — written in Rust.

`rustenv` helps developers manage `.env` files, compare differences between environments, validate configurations, mask secrets, and securely encrypt/decrypt environment files.

## Features

- **Profile Management** — Handle `.env.development`, `.env.staging`, `.env.production` easily.
- **Diffing** — Compare variables between two environments with colorized output.
- **Validation** — Scan environment files for missing values, invalid ports, malformed emails, and invalid URLs.
- **Smart Masking** — Automatically hide sensitive keys (like passwords, keys, secrets, tokens).
- **Strong Encryption** — Encrypt and decrypt `.env` files with AES-256-GCM.
- **Multi-Format Export** — Export variables to Shell (`export KEY=val`), Docker Compose, or JSON.
- **Interactive Init** — Initialize `.env` from a template, prompting you for each value.

## Installation

### 1. From Source (Cargo)
```bash
cargo install --git https://github.com/anilcan-kara/rustenv.git
```

### 2. Direct Binary Download
You can download the precompiled static binary for your platform directly from the GitHub Release assets:
- 💻 **Windows (x64)**: [rustenv-win32-x64.exe](https://github.com/anilcan-kara/rustenv/releases/download/v0.1.1/rustenv-win32-x64.exe)
- 🐧 **Linux (x64)**: [rustenv-linux-x64](https://github.com/anilcan-kara/rustenv/releases/download/v0.1.1/rustenv-linux-x64)
- 🐧 **Linux (ARM64)**: [rustenv-linux-arm64](https://github.com/anilcan-kara/rustenv/releases/download/v0.1.1/rustenv-linux-arm64)
- 🍎 **macOS (x64)**: [rustenv-darwin-x64](https://github.com/anilcan-kara/rustenv/releases/download/v0.1.1/rustenv-darwin-x64)
- 🍎 **macOS (ARM64)**: [rustenv-darwin-arm64](https://github.com/anilcan-kara/rustenv/releases/download/v0.1.1/rustenv-darwin-arm64)

## Usage

### Show Variables

```bash
rustenv show                    # Shows variables (masked by default)
rustenv show --unmask           # Shows variables unmasked
```

### Diff Environments

```bash
rustenv diff .env.staging .env.production
```

### Validate Variables

Checks for empty values, syntax rules, and common formats (`_PORT`, `_URL`, `_EMAIL`).

```bash
rustenv validate
```

### Export Variables

```bash
rustenv export --format shell
rustenv export --format docker
rustenv export --format json
```

### Encrypt & Decrypt Secrets

Securely encrypt your `.env` file before committing to source control.

```bash
rustenv encrypt .env
rustenv decrypt .env.enc
```

### Merge Environments

```bash
rustenv merge .env.base .env.local -o .env
```

### Initialize from Template

```bash
rustenv init --from .env.template --output .env --interactive
```

## License

This project is licensed under the MIT License.
