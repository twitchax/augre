[![Build and Test](https://github.com/twitchax/augre/actions/workflows/build.yml/badge.svg)](https://github.com/twitchax/augre/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/twitchax/augre/branch/main/graph/badge.svg?token=35MZN0YFZF)](https://codecov.io/gh/twitchax/augre)
[![Version](https://img.shields.io/crates/v/augre.svg)](https://crates.io/crates/augre)
[![Crates.io](https://img.shields.io/crates/d/augre?label=crate)](https://crates.io/crates/augre)
[![GitHub all releases](https://img.shields.io/github/downloads/twitchax/augre/total?label=binary)](https://github.com/twitchax/augre/releases)
[![Rust](https://img.shields.io/badge/rust-nightly-blue.svg?maxAge=3600)](https://github.com/twitchax/augre)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# augre

An LLM-powered (CodeLlama or OpenAI) local diff code review tool.

## Binary Usage

### Install

Windows:

1. Open PowerShell as an Administrator.

2. Download and insall augre:
```powershell
$ iwr -uri https://github.com/twitchax/augre/releases/latest/download/augre_x86_64-pc-windows-gnu.zip -OutFile "augre_x86_64-pc-windows-gnu.zip"
$ Expand-Archive augre_x86_64-pc-windows-gnu.zip -DestinationPath "$env:LOCALAPPDATA\Programs\augre"
```
3. Add the augre directory to your PATH.
```powershell
$augre_path = "$env:LOCALAPPDATA\Programs\augre"
$current_path = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($current_path -notlike "*$augre_path*") {
    [Environment]::SetEnvironmentVariable("PATH", "$current_path;$augre_path", "User")
    $env:PATH = "$env:PATH;$augre_path"
}
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
$ augre -h
Usage: augre [OPTIONS] [COMMAND]

Commands:
  review  Performs a code review of the current `git diff HEAD^`
  ask     Gives a response to the specified prompt
  stop    Stop all of the background services
  help    Print this message or the help of the given subcommand(s)

Options:
  -d, --data-path <DATA_PATH>  The path to the data directory [default: .augre]
  -m, --mode <MODE>            The default operation mode [default: openai]
  -y, --yes                    Whether to skip the confirmation prompt
  -h, --help                   Print help
  -V, --version                Print version
```

## Example Config

Save config as `.augre/config.toml` in the directory that needs to be reviewed

```toml
mode = "LocalGpu"
model_url = "https://huggingface.co/TheBloke/CodeLlama-13B-Instruct-GGML/resolve/main/codellama-13b-instruct.ggmlv3.Q3_K_M.bin"
cria_port = 3000
```

## License

MIT
