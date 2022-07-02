# wallpaper_scraper
Rust program that scrapes a directory for images which are suitable to be wallpapers, then saves them to another directory.

## Usage

```bash
wallpaper_scraper [OPTIONS] [DIRECTORY]
```

There are just a couple of options:

`-o, --output <output>` Directory to which images are copied (default: ./wallpapers)

`-r, --ratio <aspect ratio>` Screen aspect ratio (default: 16x9) (possible values: 4x3, 16x9)

`-t, --tolerance <tolerance>` Allowed deviation from standard aspect ratios<br>(default: med) (possible values: high, med, low)

`-h, --help` Print help information

`-V, --version` Print version information

## Installation

### Cargo

Clone the repository, then

```bash
cargo install --path path/to/wallpaper_scraper
```

Note: I have a similar program in Python but I wanted to write it in Rust... just because.
