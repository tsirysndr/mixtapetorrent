# MixtapeTorrent CLI

<p>
  <a href="https://crates.io/crates/mixtapetorrent" target="_blank">
    <img src="https://img.shields.io/crates/v/mixtapetorrent.svg" />
  </a>
   <a href="https://crates.io/crates/mixtapetorrent" target="_blank">
    <img src="https://img.shields.io/crates/dr/mixtapetorrent" />
  </a>
  <a href="https://docs.rs/mixtapetorrent" target="_blank">
    <img src="https://docs.rs/mixtapetorrent/badge.svg" />
  </a>
  <a href="LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg" />
  </a>
  <a href="https://github.com/tsirysndr/mixtapetorrent/actions/workflows/release.yml" target="_blank">
    <img alt="release" src="https://github.com/tsirysndr/mixtapetorrent/actions/workflows/release.yml/badge.svg" />
  </a>
  <a href="https://github.com/tsirysndr/mixtapetorrent/actions/workflows/rust-clippy.yml" target="_blank">
    <img alt="release" src="https://github.com/tsirysndr/mixtapetorrent/actions/workflows/rust-clippy.yml/badge.svg?branch=master" />
  </a>
</p>

<p>
<a href="https://www.buymeacoffee.com/tsiry">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-red.png" alt="Buy Me A Coffee" height="40" />
</a>
</p>

This is a command line interface and Rust library for [MixtapeTorrent](http://www.mixtapetorrent.com/).

<img width="800" src="./preview.svg">

## Installation

Simply run:

```bash
$ cargo install mixtapetorrent
```

### macOS/Linux
```bash
brew install tsirysndr/tap/mixtapetorrent
```

Or download the latest release for your platform [here](https://github.com/tsirysndr/mixtapetorrent/releases).

## Usage

```
USAGE:
    mixtapetorrent [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -j, --json       output in json format
    -V, --version    Print version information

SUBCOMMANDS:
    dj         List all the DJs
    help       Print this message or the help of the given subcommand(s)
    info       Get the details of a mixtape
    latest     Get the latest mixtapes
    popular    Get the most popular mixtapes
```
