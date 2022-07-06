use wallpaper_scraper::{config, run};

fn main() {
    let args = config::args();
    if let Err(e) = run(args) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
