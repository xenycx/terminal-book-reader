use crate::book::models::{Book, Chapter, Format};
use epub::doc::EpubDoc;
use html2text::from_read;
use std::fs;
use std::path::Path;

pub fn load_book_metadata<P: AsRef<Path>>(path: P) -> Result<Book, String> {
    let path_ref = path.as_ref();
    let ext = path_ref.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext.to_lowercase().as_str() {
        "epub" => {
            let doc =
                EpubDoc::new(path_ref).map_err(|e| format!("Failed to open EPUB: {:?}", e))?;
            let title = doc
                .mdata("title")
                .map(|m| m.value.clone())
                .unwrap_or_else(|| "Unknown Title".to_string());
            let author = doc.mdata("creator").map(|m| m.value.clone());
            let date = doc.mdata("date").map(|m| m.value.clone());
            let description = doc.mdata("description").map(|m| m.value.clone());
            Ok(Book {
                id: generate_id(path_ref),
                title,
                author,
                category: None,
                date,
                description,
                path: path_ref.to_path_buf(),
                format: Format::Epub,
                chapters: Vec::new(), // Load empty chapters for metadata
                current_chapter: 0,
                current_position: 0,
                bookmarks: vec![],
                last_read: String::new(),
                time_spent_secs: 0,
            })
        }
        "txt" => {
            let title = path_ref
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .to_string();
            Ok(Book {
                id: generate_id(path_ref),
                title,
                author: None,
                category: None,
                date: None,
                description: None,
                path: path_ref.to_path_buf(),
                format: Format::Txt,
                chapters: Vec::new(),
                current_chapter: 0,
                current_position: 0,
                bookmarks: Vec::new(),
                last_read: String::new(),
                time_spent_secs: 0,
            })
        }
        _ => Err("Unsupported file format".to_string()),
    }
}

pub fn load_book<P: AsRef<Path>>(path: P) -> Result<Book, String> {
    let path_ref = path.as_ref();
    let ext = path_ref.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext.to_lowercase().as_str() {
        "epub" => load_epub(path_ref),
        "txt" => load_txt(path_ref),
        _ => Err("Unsupported file format".to_string()),
    }
}

fn generate_id(path: &Path) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    path.to_string_lossy().hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn load_epub(path: &Path) -> Result<Book, String> {
    let mut doc = EpubDoc::new(path).map_err(|e| format!("Failed to open EPUB: {:?}", e))?;

    let title = doc
        .mdata("title")
        .map(|m| m.value.clone())
        .unwrap_or_else(|| "Unknown Title".to_string());
    let author = doc.mdata("creator").map(|m| m.value.clone());
    let date = doc.mdata("date").map(|m| m.value.clone());
    let description = doc.mdata("description").map(|m| m.value.clone());

    let mut chapters = Vec::new();

    let num_pages = doc.get_num_chapters();
    for i in 0..num_pages {
        if !doc.set_current_chapter(i) {
            return Err(format!("Failed to set chapter {}", i));
        }
        // Convert HTML to plain text using html2text
        let content_html = doc.get_current_str().map(|(c, _)| c).unwrap_or_default();
        let content_text = from_read(content_html.as_bytes(), 80).map_err(|e| e.to_string())?;

        if !content_text.trim().is_empty() {
            let ch_title = format!("Chapter {}", chapters.len() + 1);

            chapters.push(Chapter {
                title: ch_title,
                content: content_text,
            });
        }
    }

    Ok(Book {
        id: generate_id(path),
        title,
        author,
        category: None,
        date,
        description,
        path: path.to_path_buf(),
        format: Format::Epub,
        chapters,
        current_chapter: 0,
        current_position: 0,
        bookmarks: vec![],
        last_read: String::new(), // You'd put a real timestamp here
        time_spent_secs: 0,
    })
}

fn load_txt(path: &Path) -> Result<Book, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read TXT: {:?}", e))?;
    let title = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string();

    let chapters = vec![Chapter {
        title: "Full Text".to_string(),
        content,
    }];

    Ok(Book {
        id: generate_id(path),
        title,
        author: None,
        category: None,
        date: None,
        description: None,
        path: path.to_path_buf(),
        format: Format::Txt,
        chapters,
        current_chapter: 0,
        current_position: 0,
        bookmarks: Vec::new(),
        last_read: chrono::Utc::now().to_rfc3339(),
        time_spent_secs: 0,
    })
}
