use std::{fs, path::Path};

use clap::Parser;
use mdbook_fmv::fm::{Frontmatter, check_frontmatter, fix_frontmatter};
use mdbook_fmv::git::file_commit_info;
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

    /// Automatically fix issues where possible
    #[arg(long)]
    fix: bool,
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

        if cli.fix && diags.iter().any(|d| d.code == "fm::missing-frontmatter") {
            let abs_path = std::path::Path::new(&full_path).canonicalize().unwrap();
            let commit = file_commit_info(&abs_path, "%Y-%m-%d", false)
                .ok()
                .flatten();

            let title = path
                .trim_end_matches(".md")
                .split('/')
                .next_back()
                .unwrap_or("untitled");

            let fm = Frontmatter {
                title,
                author: commit
                    .as_ref()
                    .map(|c| c.author.as_str())
                    .unwrap_or("Unknown"),
                date: commit
                    .as_ref()
                    .map(|c| c.date.as_str())
                    .unwrap_or("Unknown"),
            };

            let fixed = fix_frontmatter(&content, &fm);
            match fs::write(&full_path, fixed) {
                Ok(_) => eprintln!("fixed: {full_path}"),
                Err(e) => eprintln!("error: could not write {full_path}: {e}"),
            }
        }
    }

    if total == 0 {
        println!("fmv: no issues found");
    } else {
        eprintln!("\nfmv: {total} issue(s) found");
        std::process::exit(1);
    }
}
