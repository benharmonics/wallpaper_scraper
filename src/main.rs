fn main() {
    let args = wallpaper_scraper::config::args();
    if let Err(e) = wallpaper_scraper::run(args) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
