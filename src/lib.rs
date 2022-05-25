pub mod config;

use std::{fs, io, env};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use clap::ArgMatches;

pub fn run(args: ArgMatches) -> io::Result<()> {
    if let Some(dirs) = args.values_of("DIRECTORY") {
        for dir in dirs {
            let buf = fs::canonicalize(dir)?;
            scrape_dir(buf.as_path(), &args)?;
        }
    } else {
        let buf = env::current_dir()?;
        scrape_dir(buf.as_path(), &args)?;
    }
    Ok(())
}

fn scrape_dir(path: &Path, args: &ArgMatches) -> io::Result<()> {
    let output_dir = Path::new(args.value_of("output").unwrap());
    if !output_dir.exists() {
        fs::create_dir(&output_dir)?;
    }
    // Gathering PathBufs for items in given directory, then retaining just the image files for scraping
    let image_filetypes = ["jpg", "jpeg", "png", "JPG", "JPEG", "PNG"]
        .map(|s| OsStr::new(s));
    let mut pathbufs = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<PathBuf>, _>>()
        .unwrap_or_default();
    pathbufs.retain(|buf| image_filetypes.contains(&buf.extension().unwrap_or_default()));
    // Scraping suitable images - that is, copying them into a new directory.
    for buf in &pathbufs {
        if !image_is_suitable(buf.as_path(), args) { continue; }
        fs::copy(buf, output_dir.join(buf.file_name().unwrap()))?;
    }
    Ok(())
}

// Check if image is HD and within an acceptable tolerance of a popular aspect ratio (16:9 or 4:3)
fn image_is_suitable(path: &Path, args: &ArgMatches) -> bool {
    let (width, height) = image::image_dimensions(path).unwrap_or_default();
    if width < 1920 || height < 1080 {
        return false
    }
    let aspect_ratio = width as f64 / height as f64;
    let r1 = 16.0 / 9.0;
    let r2 = 4.0 / 3.0;
    let tolerance = match args.value_of("tolerance").unwrap() {
        "high" => 0.22,
        "med" => 0.15,
        "low" => 0.08,
        _ => 0.15,
    };
    if (aspect_ratio - r1).abs() / r1 > tolerance && (aspect_ratio - r2).abs() / r2 > tolerance {
        return false
    }
    true
}