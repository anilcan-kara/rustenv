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

```bash
cargo install --path .
```

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
