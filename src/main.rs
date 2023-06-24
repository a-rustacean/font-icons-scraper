use std::{fs::{create_dir, write}, process::exit};
use font_icons_scraper::scrap_font_icons;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Invalid args");
        exit(1);
    }
    let css_url = args[1].clone();
    let output_dir = args[2].clone();
    create_dir(&output_dir).unwrap();
    let icons = scrap_font_icons(css_url).await?;
    for (name, svg) in icons {
        let output_file = format!("{}/{}.svg", output_dir, name);
        println!("Writing: {}", output_file);
        write(
            output_file,
            svg,
        )
        .unwrap();
    }
    Ok(())
}
