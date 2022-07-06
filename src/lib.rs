pub mod config;

use clap::ArgMatches;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub fn run(args: ArgMatches) -> io::Result<()> {
    match args.values_of("DIRECTORY") {
        Some(dirs) => {
            for dir in dirs {
                let buf = fs::canonicalize(dir)?;
                scrape_dir(buf.as_path(), &args)?;
            }
        }
        None => {
            let buf = env::current_dir()?;
            scrape_dir(buf.as_path(), &args)?;
        }
    }

    Ok(())
}

fn scrape_dir(path: &Path, args: &ArgMatches) -> io::Result<()> {
    // Gathering PathBufs for items in given directory, then retaining just the image files for scraping
    let image_filetypes = ["bmp", "jpeg", "jpg", "png", "psd"].map(|s| OsStr::new(s));
    let mut pathbufs: Vec<PathBuf> = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .map(|res| res.unwrap_or_default())
        .collect();
    pathbufs.retain(|buf| {
        let extension = buf.extension().unwrap_or_default().to_ascii_lowercase();
        image_filetypes.contains(&extension.as_os_str())
    });
    // Output directory - by default it's `./wallpapers` whence the program is invoked
    let output_dir = Path::new(args.value_of("output").unwrap());
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }
    // We assume your monitor is 16:9 but you can set it to 4:3 as well
    let aspect_ratio = match args.value_of("aspect ratio").unwrap() {
        "16:9" => 16.0 / 9.0,
        "4:3" => 4.0 / 3.0,
        _ => unreachable!(),
    };
    // Kind of arbitrary values. They were found by manually testing a lot of different size images.
    let tolerance = match args.value_of("tolerance").unwrap() {
        "high" => 0.22,
        "med" => 0.15,
        "low" => 0.08,
        _ => unreachable!(),
    };
    // Scraping suitable images - that is, copying them into a new directory.
    for (_i, path) in pathbufs.iter().enumerate() {
        if !image_is_suitable(path.as_path(), aspect_ratio, tolerance) {
            continue;
        }
        let path = path.file_name().unwrap_or(OsStr::new(stringify!(_i)));
        fs::copy(path, output_dir.join(path))?;
    }

    Ok(())
}

// Check if image is HD and within an acceptable tolerance of a popular aspect ratio (16:9 or 4:3)
fn image_is_suitable(path: &Path, aspect_ratio: f64, tolerance: f64) -> bool {
    let (width, height) = match imagesize::size(path) {
        Ok(dim) => (dim.width, dim.height),
        Err(e) => {
            eprintln!("Error getting image dimensions: {}", e);
            (0, 0)
        }
    };
    // size check - images must be HD
    if width < 1920 || height < 1080 {
        return false;
    }
    // aspect ratio check - images should be 16:9 (with some wiggle room)
    let image_aspect_ratio = width as f64 / height as f64;
    if (image_aspect_ratio - aspect_ratio).abs() / aspect_ratio > tolerance {
        return false;
    }

    true
}
