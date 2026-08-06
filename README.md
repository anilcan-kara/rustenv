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

## Why rustenv?

Managing `.env` files is painful. You share them over Slack, forget to update `.env.example`, commit secrets by accident, or have no idea what changed between environments. **rustenv** gives you a proper CLI for inspecting, comparing, validating, encrypting, and exporting your environment files.

---

## Features

- 👁️ **Show** — pretty-print any `.env` file with automatic secret masking
- 🔍 **Diff** — compare two `.env` files side-by-side, see added/removed/changed keys
- ✅ **Validate** — check syntax, detect empty values, find duplicates
- 📤 **Export** — convert `.env` to shell, Docker, or JSON format
- 🔐 **Encrypt** — AES-256 encrypt your `.env` file into a `.env.enc`
- 🔓 **Decrypt** — decrypt a `.env.enc` back to plaintext
- 🎭 **Secret masking** — auto-masks values for keys containing `SECRET`, `TOKEN`, `KEY`, `PASS`, `PWD`

---

## Usage

### `show` — Display an env file
```bash
rustenv show                    # reads .env in current dir
rustenv show .env.production    # specific file
rustenv show --unmask           # show real values (no masking)
```

### `diff` — Compare two env files
```bash
rustenv diff .env .env.production
```

### `validate` — Check env file syntax
```bash
rustenv validate
```

### `export` — Convert to different formats
```bash
rustenv export --format shell           # export KEY="value" (sourceable)
rustenv export --format docker          # --env KEY=value flags for docker run
rustenv export --format json            # { "KEY": "value" }
```

### `encrypt` / `decrypt` — AES-256 encryption
```bash
rustenv encrypt .env                    # creates .env.enc (prompts for password)
rustenv decrypt .env.enc -o .env        # decrypt back
```

---

## CLI Reference

```
rustenv <COMMAND>

Commands:
  show      Display an env file with masked secrets
  diff      Compare two env files
  validate  Validate env file syntax and values
  export    Export env file in various formats
  encrypt   Encrypt a .env file with AES-256
  decrypt   Decrypt a .env.enc file
  help      Print help for a command
```

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All PRs welcome.

---

## License

MIT © [Anilcan Kara](https://github.com/anilcan-kara)
