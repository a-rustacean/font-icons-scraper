use args::FontIconsScraperArgs;
use clap::Parser;
use font_icons_scraper::scrap_font_icons;
mod args;
use std::{fs::create_dir, io::ErrorKind, path::Path, process::exit};

fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) {
    if let Err(err) = std::fs::write(path.as_ref(), contents) {
        eprintln!(
            "Unable to write file \"{}\": {}",
            path.as_ref().to_str().unwrap(),
            err
        );
        exit(1);
    }
}

#[tokio::main]
async fn main() {
    let args = FontIconsScraperArgs::parse();
    if let Err(error) = create_dir(&args.output_dir) {
        if error.kind() != ErrorKind::AlreadyExists {
            eprintln!(
                "Unable to create directory \"{}\"",
                args.output_dir.to_str().unwrap_or("output")
            );
            exit(1);
        }
    };
    let icons = match scrap_font_icons(args.url.clone(), args.depth.unwrap_or(0)).await {
        Ok(v) => v,
        Err(err) => {
            eprintln!(
                "Unable to scrap font icons from url \"{}\": {}",
                args.url, err
            );
            exit(1);
        }
    };

    for (name, svg) in icons {
        let output_file = format!(
            "{}/{}.svg",
            args.output_dir.to_str().unwrap_or("output"),
            name
        );
        println!("Writing: {}", output_file);
        write(output_file, svg);
    }
}
