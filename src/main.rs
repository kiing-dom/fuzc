mod core;

use core::search;

fn main() {
    let curr_dir = std::env::current_dir().expect("Failed to get current directory!");
    let file_paths = core::discover::find_source_files(&curr_dir, "java");
    let files = core::source::load_files(&file_paths);
    let comments = core::engine::extract_comments(&files);

    let query = "comment";
    let results = search::search(&comments, query);

    println!("Searc results for {}: {} matches", query, results.len());
    for result in results {
        println!("{}:{}: {}", result.file_name, result.line, result.text.trim());
    }
}