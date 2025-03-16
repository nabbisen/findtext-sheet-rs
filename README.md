# Find Text Sheet

[![crates.io](https://img.shields.io/crates/v/findtext_sheet?label=latest)](https://crates.io/crates/findtext_sheet)
[![Documentation](https://docs.rs/findtext_sheet/badge.svg?version=latest)](https://docs.rs/findtext_sheet)
[![Dependency Status](https://deps.rs/crate/findtext_sheet/latest/status.svg)](https://deps.rs/crate/findtext_sheet)
[![Releases Workflow](https://github.com/nabbisen/findtext-sheet-rs/actions/workflows/release-executable.yaml/badge.svg)](https://github.com/nabbisen/findtext-sheet-rs/actions/workflows/release-executable.yaml)
[![License](https://img.shields.io/github/license/nabbisen/findtext-sheet-rs)](https://github.com/nabbisen/findtext-sheet-rs/blob/main/LICENSE)

## Summary

Search text in SpreadSheet

## Development

First, add dependency:

```sh
cargo add findtext_sheet
```

Usage:

```rust
use findtext_sheet::search;

fn awesome_fn(keyword: &str, filepath: &str) {
    let ret = search(keyword, filepath);
}
```

## Executable Usage

Available in [Assets](https://github.com/nabbisen/findtext-sheet-rs/releases/latest) in Releases. Cross-platform supported.

```sh
findtext_sheet <keyword> <filepath>
# will print out like:
# Sheet1: (4, 1) = hej
# which means `<sheet-name>: (cell <row>, <column>) = <cell value>`
```

## Acknowledgements

Depends on:

- [tafia](https://github.com/tafia)'s [calamine](https://github.com/tafia/calamine) and [quick-xml](https://github.com/tafia/quick-xml)
