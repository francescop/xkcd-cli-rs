use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ComicResponse {
    pub month: String,
    pub num: u32,
    pub link: String,
    pub year: String,
    pub news: String,
    pub safe_title: Option<String>,
    pub transcript: String,
    pub alt: String,
    pub img: String,
    pub title: String,
    pub day: String,
}

impl ComicResponse {
    pub fn formatted_date(&self) -> String {
        format!("{}-{}-{}", self.day, self.month, self.year)
    }
}
