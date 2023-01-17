# markov-chain-generator-jp-cli

![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/RShirohara/markov-chain-generator-jp-cli/release.yml?logo=github) ![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/RShirohara/markov-chain-generator-jp-cli?logo=github) ![GitHub](https://img.shields.io/github/license/RShirohara/markov-chain-generator-jp-cli)

Japanese markov chain generator.

## Installation

Download binary from [Releases](https://github.com/RShirohara/markov-chain-generator-jp-cli/releases).

## Build

Default toolchain: `stable-x86_64-unknown-linux-musl`

```bash
cargo build --release
```

## Usage

### Specify a single file

```bash
markov -i /path/to/source -s 3 -r 100
```

### Specify multiple files

```bash
cat /path/to/first/file /path/to/second/file | markov -s 3 -r 100
```

```bash
markov -i <(cat /path/to/first/file /path/to/second/file) -s3 -r100
```
