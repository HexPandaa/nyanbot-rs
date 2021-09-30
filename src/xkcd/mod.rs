use chrono::prelude::*;

// type Result<T> = std::result::Result<T, XkcdError>;

#[derive(Debug, Clone)]
enum XkcdError {
    DownloadError,
    JsonError,
}

impl std::fmt::Display for XkcdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XkcdError::DownloadError => {
                write!(f, "Error downloading comic, maybe the id is invalid?")
            }
            XkcdError::JsonError => {
                write!(f, "Error decoding JSON response")
            }
        }
    }
}

/// The representation of a comic
struct Comic {
    title: String,
    safe_title: String,
    num: u32,
    date: Date<Utc>,
    img_url: String,
    alt: String,
    transcript: String,
    news: String,
}
