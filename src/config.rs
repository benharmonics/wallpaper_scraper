use clap::{arg, Arg, ArgMatches, Command};

const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");

pub fn args() -> ArgMatches {
    Command::new(PROGRAM_NAME)
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
                .multiple_values(false)
                .default_value("./wallpapers")
                .forbid_empty_values(true)
                .help("Directory to which images are copied"),
        )
        .arg(
            Arg::new("tolerance")
                .short('t')
                .long("tolerance")
                .alias("tol")
                .takes_value(true)
                .multiple_values(false)
                .default_value("med")
                .possible_values(["high", "med", "low"])
                .forbid_empty_values(true)
                .help("Allowed deviation from standard aspect ratios"),
        )
        .arg(
            Arg::new("aspect ratio")
                .short('r')
                .long("ratio")
                .takes_value(true)
                .multiple_values(false)
                .default_value("16:9")
                .possible_values(["4:3", "16:9"])
                .forbid_empty_values(true)
                .help("Screen aspect ratio"),
        )
        .get_matches()
}
