use epub::doc::EpubDoc;

fn main() {
    let path = "test.epub";
    if let Ok(mut doc) = EpubDoc::new(path) {
        println!("Chapters: {}", doc.get_num_pages());
    }
}
