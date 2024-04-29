# Rofi Bookmarks

**Work-in-Progress**

A utility to manage web bookmarks from [Rofi].

## Usage

Specify a path to a `bookmarks.toml` file in the `ROFI_BOOKMARKS_PATH` environment variable.

```bash
cargo build --release
ROFI_BOOKMARKS_PATH=$PWD/bookmarks.toml rofi -show bookmarks -modi bookmarks:$PWD/target/release/rofi-bookmarks
```

The `bookmarks.toml` file consists of *items* and *groups*.

```
[example-item]
# Optional: alternative display title instead of `example-item`
title = "Example Item"
# Manditory: URL the item points to
url = "https://example.com"
# Optional: set of keywords that can be searched for in Rofi
keywords = [ "a", "list", "of", "words", "or sentences" ]

[example-group]
# Optional: alternative display title instead of `example-group`
title = "Example Group"

"Google".url = "https://google.com"
[bing]
title = "Bing"
url = "https://bing.com"
keywords = [ "search", "microsoft" ]
```

## License

Licensed under an MIT license.

[Rofi]: https://davatorium.github.io/rofi/