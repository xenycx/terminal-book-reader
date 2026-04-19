use std::env;

fn main() {
    let url = "https://www.gutenberg.org/ebooks/1342.epub.images"; // Pride and prejudice
    let path = env::current_dir().unwrap().join("test_book.epub");
    let bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    std::fs::write(&path, bytes).unwrap();
    println!("Downloaded to {:?}", path);
}
