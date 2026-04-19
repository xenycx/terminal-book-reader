use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Format {
    Epub,
    Txt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub position: usize,
    pub timestamp: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub category: Option<String>,
    pub date: Option<String>,
    pub description: Option<String>,
    pub path: PathBuf,
    pub format: Format,
    pub chapters: Vec<Chapter>,
    pub current_chapter: usize,
    pub current_position: usize,
    pub bookmarks: Vec<Bookmark>,
    pub last_read: String,
    #[serde(default)]
    pub time_spent_secs: u64,
}
