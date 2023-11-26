# Code Status: Multi Repository Git Status
Inspired by a fear that there is some code somewhere on my computer that has not been commited and pushed. Written in Rust, this cli program recursively looks for git repositories from a root folder (Like `~/code`!) and gives a status overview for each.

# Installation
Currently this has not been uploaded to crates.io, so installation is from source.
```sh
$ git clone https://github.com/WilliamMoolman/code-status
$ cd code-status
$ cargo build r
$ cp target/release/code-status ~/.local/bin/
$ code-status -V
code-status 0.1.0
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
  -s, --summary  Display summary format
  -c, --clean    Display clean repositories
  -t, --tree     Display in tree form
  -h, --help     Print help
  -V, --version  Print version

$ code-status -u -l -c ~/code
rustlings                   -M---- https://github.com/rust-lang/rustlings
moolman-dev                 N-D--- git@github.com:WilliamMoolman/moolman-dev.git
bingo                       ------ git@github.com:WilliamMoolman/BingoCards.git
QuickBudget                 NM---- git@github.com:WilliamMoolman/QuickBudget.git

$ code-status ~/code
1 clean repositories
3 repositories in dirty state, of which:
    2 repositories with modified files
    3 repositories with untracked files
```