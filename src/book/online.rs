use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct GutendexResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<OnlineBook>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OnlineBook {
    pub id: u32,
    pub title: String,
    pub authors: Vec<Author>,
    pub formats: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Author {
    pub name: String,
}

impl OnlineBook {
    pub fn epub_url(&self) -> Option<String> {
        self.formats.get("application/epub+zip").cloned()
    }
}
