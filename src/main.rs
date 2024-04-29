use std::io;
use std::process::exit;

struct Bookmark<'a> {
    title: &'a str,
    url: &'a str,
    extra: &'a str,
}

macro_rules! exit_error {
    ($exit_code:literal, $fmt:literal$(, $arg:expr)* $(,)?) => {{
        eprintln!($fmt$(, $arg)*);
        if std::env::var("ROFI_RETV").is_ok() {
            println!(concat!("\x00message\x1fError: ", $fmt)$(, $arg)*);
        }
        exit($exit_code)
    }};
}

fn main() -> io::Result<()> {
    let Ok(bookmarks_file_path) = std::env::var("ROFI_BOOKMARKS_PATH") else {
        exit_error!(1, "Environment variable 'ROFI_BOOKMARKS_PATH' not set")
    };

    let Ok(bookmarks_file) = std::fs::read_to_string(&bookmarks_file_path) else {
        exit_error!(1, "Failed to read bookmarks file at {}", bookmarks_file_path)
    };

    let mut args = std::env::args();

    args.next().expect("No argv[0]");

    let mut bookmark_iter = bookmarks_file
        .lines()
        .enumerate()
        .filter(|(_, line)| line.trim_start().starts_with('#'))
        .filter(|(_, line)| !line.is_empty())
        .map(|(nr, line)| (nr + 1, line))
        .map(|(nr, line)| {
            let Some((title, line)) = line.split_once("::") else {
                exit_error!(1, "Missing '::' on line #{} in {}", nr, bookmarks_file_path)
            };

            let (url, extra) = line
                .trim_start()
                .split_once(char::is_whitespace)
                .unwrap_or((line, ""));

            if url.is_empty() {
                exit_error!(1, "Missing URL on line #{} in {}", nr, bookmarks_file_path)
            };

            let title = title.trim();
            let url = url.trim_end();
            let extra = extra.trim();

            Bookmark { title, url, extra }
        });

    match args.next() {
        None => {
            use io::Write;
            let mut stdout = io::stdout();

            for Bookmark { title, url, extra } in bookmark_iter {
                writeln!(stdout, "{title}\x00meta\x1f{url},{extra}\x1f")?;
            }
        }
        Some(selector) => {
            let Some(Bookmark { url, .. }) =
                bookmark_iter.find(|bookmark| &bookmark.title == &selector)
            else {
                exit_error!(1, "Failed to find '{}' in bookmarks", selector)
            };

            if let Err(err) = open::that_detached(&url[..]) {
                exit_error!(1, "Failed to open URL '{}'. Reason: {}", url, err)
            }
        }
    }

    Ok(())
}
