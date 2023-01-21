# markov-chain-generator-jp-cli

[![Latest release status](https://img.shields.io/github/actions/workflow/status/RShirohara/markov-chain-generator-jp-cli/release.yml?logo=github)](https://github.com/RShirohara/markov-chain-generator-jp-cli/actions/workflows/release.yml) [![Latest release version](https://img.shields.io/github/v/release/RShirohara/markov-chain-generator-jp-cli?logo=github)](https://github.com/RShirohara/markov-chain-generator-jp-cli/releases/latest) [![License](https://img.shields.io/github/license/RShirohara/markov-chain-generator-jp-cli)](./LICENSE)

Japanese markov chain generator.

## Installation

Download binary from [Releases](https://github.com/RShirohara/markov-chain-generator-jp-cli/releases).

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

## Help

```text
Japanese markov chain generator.

Usage: markov [OPTIONS]

Options:
  -i, --input <INPUT>            Input text file path
  -s, --state-size <STATE_SIZE>  Number of words in the model's state [default: 2]
  -r, --repeat <REPEAT>          Number of times to repeat the output [default: 100]
  -h, --help                     Print help information
  -V, --version                  Print version information
```

## Build

Default toolchain: `stable-x86_64-unknown-linux-musl`

```bash
cargo build --release
```
