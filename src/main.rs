use std::{fs, path::Path};

use clap::Parser;
use mdbook_fmv::summary::parse_summary;

#[derive(Parser)]
#[command(name = "fmv", about = "mdBook frontmatter & content validator")]
struct Cli {
    /// Check frontmatter fields only
    #[arg(long)]
    fm: bool,

    /// Check HTML structure only
    #[arg(long)]
    html: bool,
}

fn main() {
    let cli = Cli::parse();

    if !Path::new("book.toml").exists() {
        eprintln!("error: no book.toml found. Run fmv from your book root");
        std::process::exit(1);
    }

    let summary = match fs::read_to_string("src/SUMMARY.md") {
        Ok(s) => s,
        Err(_) => {
            eprintln!("error: could not read src/SUMMARY.md");
            std::process::exit(1);
        }
    };

    let paths = parse_summary(&summary);
    println!("found {} chapters", paths.len());
    for path in &paths {
        println!("  {path}");
    }
}
