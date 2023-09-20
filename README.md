[![Build and Test](https://github.com/twitchax/augre/actions/workflows/build.yml/badge.svg)](https://github.com/twitchax/augre/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/twitchax/augre/branch/main/graph/badge.svg?token=35MZN0YFZF)](https://codecov.io/gh/twitchax/augre)
[![Version](https://img.shields.io/crates/v/augre.svg)](https://crates.io/crates/augre)
[![Crates.io](https://img.shields.io/crates/d/augre?label=crate)](https://crates.io/crates/augre)
[![GitHub all releases](https://img.shields.io/github/downloads/twitchax/augre/total?label=binary)](https://github.com/twitchax/augre/releases)
[![Documentation](https://docs.rs/augre/badge.svg)](https://docs.rs/augre)
[![Rust](https://img.shields.io/badge/rust-nightly-blue.svg?maxAge=3600)](https://github.com/twitchax/augre)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# augre

An LLM-powered (CodeLlama or OpenAI) local diff code review tool.

## Binary Usage

### Install

Windows:

```powershell
$ iwr https://github.com/twitchax/augre/releases/latest/download/augre_x86_64-pc-windows-gnu.zip
$ Expand-Archive augre_x86_64-pc-windows-gnu.zip -DestinationPath C:\Users\%USERNAME%\AppData\Local\Programs\augre
```

Mac OS (Apple Silicon):

```bash
$ curl -LO https://github.com/twitchax/augre/releases/latest/download/augre_aarch64-apple-darwin.zip
$ unzip augre_aarch64-apple-darwin.zip -d /usr/local/bin
$ chmod a+x /usr/local/bin/augre
```

Linux:

```bash
$ curl -LO https://github.com/twitchax/augre/releases/latest/download/augre_x86_64-unknown-linux-gnu.zip
$ unzip augre_x86_64-unknown-linux-gnu.zip -d /usr/local/bin
$ chmod a+x /usr/local/bin/augre
```

Cargo:

```bash
$ cargo install augre
```

### Help Docs

```bash
```

## Feature Flags

## Test

## Bench

## License

MIT