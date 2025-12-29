mod core;

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
}

fn main() {
    let args = Args::parse();
    let query = &args.query;
    let directory = Path::new(&args.directory);
    
    let file_paths = core::discover::find_source_files(&directory, "java");
    let files = core::source::load_files(&file_paths);
    let comments = core::engine::extract_comments(&files);

    let results = search::search(&comments, query);

    println!("Search results for {}: {} matches", query, results.len());
    for result in results {
        println!("{}:{}: {}", result.file_name, result.line, result.text.trim());
    }
}