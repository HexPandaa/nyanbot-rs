use chrono::prelude::*;

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
