use std::{fs, path::Path};

use clap::Parser;
use mdbook_fmv::fm::check_frontmatter;
use mdbook_fmv::html::check_html;
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

    let Ok(summary) = fs::read_to_string("src/SUMMARY.md") else {
        eprintln!("error: could not read src/SUMMARY.md");
        std::process::exit(1);
    };
    // let summary = match fs::read_to_string("src/SUMMARY.md") {
    //     Ok(s) => s,
    //     Err(_) => {
    //         eprintln!("error: could not read src/SUMMARY.md");
    //         std::process::exit(1);
    //     }
    // };

    let paths = parse_summary(&summary);
    let run_fm = cli.fm || !cli.html;
    let run_html = cli.html || !cli.fm;
    let mut total = 0;

    for path in &paths {
        let full_path = format!("src/{path}");

        let Ok(content) = fs::read_to_string(&full_path) else {
            eprintln!("error: could not read {full_path}");
            continue;
        };

        let mut diags = Vec::new();
        if run_fm {
            diags.extend(check_frontmatter(&content));
        }
        if run_html {
            diags.extend(check_html(&content));
        }

        for diag in &diags {
            eprintln!(
                "warning[{}]: {}\n  --> src/{}",
                diag.code, diag.message, path
            );
            total += 1;
        }
    }

    if total == 0 {
        println!("fmv: no issues found");
    } else {
        eprintln!("\nfmv: {total} issue(s) found");
        std::process::exit(1);
    }
}
