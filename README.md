# Code Status: Multi Repository Git Status
Inspired by a fear that there is some code somewhere on my computer that has not been commited and pushed. Written in Rust, this cli program recursively looks for git repositories from a root folder (Like `~/code`!) and gives a status overview for each.

# Installation
Currently this has not been uploaded to crates.io, so installation is from source.
```sh
git clone https://github.com/WilliamMoolman/code-status
cd code-status
cargo build r
cp target/release/code-status ~/.local/bin/
```
> [!TIP]
> Make sure ~/.local/bin is on your PATH!

# Usage
```sh
$ code-status -h                             
A CLI tool to get the status of many git repositories

Usage: code-status [OPTIONS] [PATH]

Arguments:
  [PATH]  The root level folder to begin searching [default: .]

Options:
  -u, --url      Display origin url
  -h, --help     Print help
  -V, --version  Print version

$ code-status -u ~/code
rustlings                   -M---- https://github.com/rust-lang/rustlings
moolman-dev                 N-D--- git@github.com:WilliamMoolman/moolman-dev.git
bingo                       ------ git@github.com:WilliamMoolman/BingoCards.git
QuickBudget                 NM---- git@github.com:WilliamMoolman/QuickBudget.git
```