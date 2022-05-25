# wallpaper_scraper
Rust program that scrapes a directory for images which are suitable to be wallpapers, then saves them to another directory.

I have a similar program in Python but I wanted to write it in Rust... just because.

## Usage

```bash
wallpaper_scraper [OPTIONS] [DIRECTORY]
```

There are just a couple of options:

`-o, --output <output>` Directory to which images are copied (default: ./wallpapers)

`-t, --tolerance <tolerance>` Allowed deviation from standard aspect ratios<br>(default: med) (possible values: high, med, low)

## Installation

### Cargo

Clone the repository, then

```bash
cargo install --path ./wallpaper_scraper
```
