# markov-chain-generator-jp-cli

![LICENSE](https://img.shields.io/github/license/RShirohara/markov-chain-generator-jp-cli)

Japanese markov chain generator.

## Build

```bash
cargo build -r
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
