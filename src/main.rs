use std::fs;

fn main() {
    let path = "example.java";
    let contents = fs::read_to_string(path)
        .expect("Failed to read file");

    println!("{}", contents);
}
