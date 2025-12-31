mod core;
mod tui;

use clap::Parser;
use std::path::Path;
use core::search;

#[derive(Parser)]
#[command(name = "comment-indexer")]
#[command(about = "a fuzzy finder for code comments")]
struct Args {
    query: String, // the search query to find in the comments

    #[arg(short, long, default_value = ".")]
    directory: String, // directory to search (default: current directory)

    #[arg(long)]
    strict: bool,
}

fn main() {
    let args = Args::parse();
    let directory = Path::new(&args.directory);

    let search_mode = if args.strict {
        search::SearchMode::And
    } else {
        search::SearchMode::Or
    };
    
    let file_paths = core::discover::find_source_files(&directory, "java");
    let files = core::source::load_files(&file_paths);
    let comments = core::engine::extract_comments(&files);

    let results = search::search(&comments, &args.query, search_mode);

    println!("Search results for {}: {} matches", &args.query, results.len());
    for result in results {
        println!("{}:{}: {}", result.file_name, result.line, result.text.trim());
    }
}