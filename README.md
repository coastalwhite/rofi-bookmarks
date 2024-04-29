# Rofi Bookmarks

**Work-in-Progress**

A utility to manage web bookmarks from [Rofi].

## Usage

Specify a path to a `book.marks` file in the `ROFI_BOOKMARKS_PATH` environment variable.

```bash
cargo build --release
ROFI_BOOKMARKS_PATH=$PWD/book.marks rofi -show bookmarks -modi bookmarks:$PWD/target/release/rofi-bookmarks
```

The `book.marks` contains lines which each contain a bookmark.

```
# Lines that start with a '#' are ignored.
#
# Each bookmark needs at least a unique title and a URL separated by "::".
# Optionally after the URL, additional search terms can be specified, delimited
# by a comma.
#
# Some examples:
Google                   :: https://google.com                 search
DuckDuckGo               :: https://duckduckgo.com             search

GitHub                   :: https://github.com                 code

# Rust related stuff
Rust Standard Library    :: https://doc.rust-lang.org/std/
Rust Crate Documentation :: https://docs.rs/

# Nix related stuff
Nix Packages             :: https://search.nixos.org/packages
NixOS Options            :: https://search.nixos.org/options 
```

## License

Licensed under an MIT license.

[Rofi]: https://davatorium.github.io/rofi/