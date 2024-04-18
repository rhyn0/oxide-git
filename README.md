# Oxide Git

This is a fun rewrite of [Git](https://git-scm.org) using Rust. It attempts to be a CLI program to be similar to Git to increase understanding of the internal workings and terminology of Git.

## CLI

### Installation

Clone this repo, build the target and then a new CLI program `og` can replace the main commands of `git`.

```bash
git clone ...
cd oxide-git
cargo build --release
# this outputs into `target/release` folder
target/release/og init # other basic commands etc.
```

## Behavior

To not interfere with actual `git` (especially inside this repo) we will replace any `git` names with `ogit`. So version history controlled by `oxide-git` is under the `.ogit` directory.

## Developing

### Linting and Formatting

This repository uses [pre-commit](https://pre-commit.com/) to run format, lint and check for CI prior to commits and merges.

That job will run `clippy` linting and `rustfmt` for formatting.

### Helpful Resources

- [Object Storage](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects#-Object-Storage)
- [The Docs](https://git-scm.com/docs)
- [Git Repository](https://github.com/git/git)
