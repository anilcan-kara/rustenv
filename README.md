<div align="center">

<h1>rustenv</h1>

<p><strong>A fast, secure environment variable and secret management tool — written in Rust.</strong></p>

[![Crates.io](https://img.shields.io/crates/v/rustenv?style=flat-square&color=fc8d62)](https://crates.io/crates/rustenv)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/anilcan-kara/rustenv/release.yml?style=flat-square)](https://github.com/anilcan-kara/rustenv/actions)
[![GitHub Release](https://img.shields.io/github/v/release/anilcan-kara/rustenv?style=flat-square&color=8be04e)](https://github.com/anilcan-kara/rustenv/releases)

```bash
rustenv show          # view .env with masked secrets
rustenv diff .env .env.production   # compare two env files
rustenv encrypt .env  # AES-256 encrypted .env.enc
```

</div>

---

## Installation

### ⚡ Quick Install (Linux / macOS)
```bash
curl -fsSL https://raw.githubusercontent.com/anilcan-kara/rustenv/master/install.sh | sh
```

### Cargo
```bash
cargo install rustenv
```

---

## Features

- 👁️ **Show** — pretty-print any `.env` file with automatic secret masking
- 🔍 **Diff** — compare two `.env` files side-by-side, see added/removed/changed keys
- ✅ **Validate** — check syntax, detect empty values, find duplicates, validate booleans
- 📤 **Export** — convert `.env` to shell, Docker, or JSON format
- 🔐 **Encrypt** — AES-256 encrypt your `.env` file into a `.env.enc`
- 🔓 **Decrypt** — decrypt a `.env.enc` back to plaintext

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All PRs welcome.

---

## License

MIT © [Anilcan Kara](https://github.com/anilcan-kara)
