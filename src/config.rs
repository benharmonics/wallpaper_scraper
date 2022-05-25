use clap::{arg, Arg, Command, ArgMatches};

pub fn args() -> ArgMatches {
    Command::new("wallpaper_scraper")
        .version("0.1")
        .author("benharmonics")
        .about("Scrapes a directory for HD images suitable to be wallpapers")
        .arg(arg!([DIRECTORY] ... "One or more directories to be scraped"))
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .aliases(&["dest", "destination"])
                .takes_value(true)
                .default_value("./wallpapers")
                .forbid_empty_values(true)
                .help("Directory to which images are copied")
        )
        .arg(
            Arg::new("tolerance")
                .short('t')
                .long("tolerance")
                .alias("tol")
                .takes_value(true)
                .default_value("med")
                .possible_values(["high", "med", "low"])
                .help("Allowed deviation from standard aspect ratios")
        )
        .get_matches()
}