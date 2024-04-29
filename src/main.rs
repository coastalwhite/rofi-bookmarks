use indexmap::IndexMap;
use std::io;

mod file {
    use indexmap::IndexMap;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct BookmarksFile(pub IndexMap<String, BookmarkItem>);

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum BookmarkItem {
        Item(BookmarkMeta),
        Group(BookmarkGroup),
    }

    #[derive(Deserialize, Debug)]
    pub struct BookmarkGroup {
        pub title: Option<String>,
        #[serde(flatten)]
        pub items: IndexMap<String, BookmarkItem>,
    }

    #[derive(Deserialize, Debug)]
    pub struct BookmarkMeta {
        pub title: Option<String>,
        pub url: String,
        #[serde(default)]
        pub keywords: Vec<String>,
    }
}

struct Bookmarks {
    items: Vec<BookmarkGroupItem>,
}

struct BookmarkGroup {
    title: String,
    children: Vec<BookmarkGroupItem>,
}

enum BookmarkGroupItem {
    Group(BookmarkGroup),
    Item(BookmarkItem),
}

struct BookmarkItem {
    title: String,
    url: String,
    keywords: Vec<String>,
}

impl From<file::BookmarksFile> for Bookmarks {
    fn from(f: file::BookmarksFile) -> Bookmarks {
        fn from_map(map: IndexMap<String, file::BookmarkItem>) -> Vec<BookmarkGroupItem> {
            let mut children = Vec::with_capacity(map.len());

            for (title, item) in map.into_iter() {
                children.push(from_item(title, item));
            }

            children
        }

        fn from_item(item_title: String, item: file::BookmarkItem) -> BookmarkGroupItem {
            match item {
                file::BookmarkItem::Item(file::BookmarkMeta {
                    title,
                    url,
                    keywords,
                }) => {
                    let title = title.unwrap_or(item_title);
                    BookmarkGroupItem::Item(BookmarkItem {
                        title,
                        url,
                        keywords,
                    })
                }
                file::BookmarkItem::Group(file::BookmarkGroup { title, items }) => {
                    let title = title.unwrap_or(item_title);
                    let mut children = Vec::with_capacity(items.len());

                    for (item_title, item) in items.into_iter() {
                        children.push(from_item(item_title, item));
                    }

                    BookmarkGroupItem::Group(BookmarkGroup { title, children })
                }
            }
        }

        let items = from_map(f.0);
        Bookmarks { items }
    }
}

enum Title<'a> {
    None,
    Title(&'a str),
}

fn print_items(title: Title<'_>, items: &'_ [BookmarkGroupItem]) -> io::Result<()> {
    use io::Write;

    let mut stdout = io::stdout();

    if let Title::Title(title) = title {
        writeln!(stdout, "\x00message\x1f{title}")?;
    }

    for item in items {
        match item {
            BookmarkGroupItem::Item(BookmarkItem {
                title,
                url,
                keywords,
            }) => {
                write!(stdout, "{title}\x00meta\x1f")?;
                for keyword in keywords.iter() {
                    write!(stdout, "{keyword},")?;
                }
                stdout.write(&[0x1f])?;
            }
            BookmarkGroupItem::Group(BookmarkGroup { title, children }) => {
                write!(stdout, "{title}\x00icon\x1ffolder")?;
            }
        }

        writeln!(stdout)?;
    }

    Ok(())
}

fn main() {
    let Ok(bookmarks_file_path) = std::env::var("ROFI_BOOKMARKS_PATH") else {
        eprintln!("Environment variable 'ROFI_BOOKMARKS_PATH' not set.");
        std::process::exit(1)
    };

    let Ok(bookmarks_file) = std::fs::read_to_string(&bookmarks_file_path) else {
        eprintln!("Failed to read bookmarks file at {bookmarks_file_path}.");
        std::process::exit(1)
    };

    let Ok(bookmarks) = toml::from_str::<file::BookmarksFile>(&bookmarks_file) else {
        eprintln!("Failed to read bookmarks file at {bookmarks_file_path}.");
        std::process::exit(1)
    };

    let bookmarks = Bookmarks::from(bookmarks);

    let mut args = std::env::args();

    args.next().expect("No argv[0]");

    let Some(selector) = args.next() else {
        print_items(Title::None, &bookmarks.items).expect("Failed to write");
        std::process::exit(0)
    };

    let mut iterator_stack = Vec::with_capacity(8);

    iterator_stack.push(bookmarks.items.iter());

    loop {
        let Some(iterator) = iterator_stack.last_mut() else {
            eprintln!("Selector '{selector}' not found!");
            std::process::exit(1)
        };

        let Some(item) = iterator.next() else {
            iterator_stack.pop();
            continue;
        };

        use BookmarkGroupItem as I;
        match item {
            I::Item(BookmarkItem {
                title,
                url,
                keywords,
            }) => {
                if title != &selector {
                    continue;
                }

                if let Err(err) = open::that(url) {
                    eprintln!("Failed to open url. Reason: {err}");
                }
            }
            I::Group(BookmarkGroup { title, children }) => {
                if title != &selector {
                    iterator_stack.push(children.iter());
                    continue;
                }

                print_items(Title::Title(&title), &children).expect("Failed to write");
            }
        }

        break;
    }
}
