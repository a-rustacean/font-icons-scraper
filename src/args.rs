use std::path::PathBuf;

use clap::Parser;
use url::Url;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FontIconsScraperArgs {
    /// the url of the css file which contains all the icons
    pub url: Url,
    /// the output path
    pub output_dir: PathBuf,
    /// the depth of recursive scraping
    #[clap(short, long)]
    pub depth: Option<usize>,
}
