use chrono::prelude::*;
use std::fmt::Formatter;

// type Result<T> = std::result::Result<T, XkcdError>;

#[derive(Debug, Clone)]
pub enum XkcdError {
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
pub struct Comic {
    pub title: String,
    pub safe_title: String,
    pub num: u32,
    pub date: Date<Utc>,
    pub img_url: String,
    pub alt: String,
    pub transcript: String,
    pub news: String,
    pub link: String,
}

impl Comic {
    /// Returns the comic of the given number
    pub fn from_num(num: u32) -> Option<Comic> {
        let json = utils::download_num(num).ok()?;
        Some(utils::json_to_comic(json))
    }

    /// Returns the current comic
    pub fn current() -> Option<Comic> {
        let json = utils::download_current().ok()?;
        Some(utils::json_to_comic(json))
    }
}

mod utils {
    use super::XkcdError;
    use crate::xkcd::Comic;
    use chrono::{Date, TimeZone, Utc};
    use serde::Deserialize;

    const BASE_URL: &str = "https://xkcd.com";

    /// Returns a chrono::Date object from the day, month and year.
    fn parse_date(day: String, month: String, year: String) -> Date<Utc> {
        let uday: u32 = day.parse().unwrap();
        let umonth: u32 = month.parse().unwrap();
        let iyear: i32 = year.parse().unwrap();

        Utc.ymd(iyear, umonth, uday)
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct JsonData {
        pub(crate) title: String,
        pub(crate) safe_title: String,
        pub(crate) num: u32,
        pub(crate) img: String,
        pub(crate) alt: String,
        pub(crate) transcript: String,
        pub(crate) news: String,
        pub(crate) link: String,
        pub(crate) day: String,
        pub(crate) month: String,
        pub(crate) year: String,
    }

    pub(crate) fn download_num(num: u32) -> Result<JsonData, XkcdError> {
        let url: String = format!("{}/{}/info.0.json", BASE_URL, num);
        download_url(url)
    }

    pub(crate) fn download_current() -> Result<JsonData, XkcdError> {
        let url: String = format!("{}/info.0.json", BASE_URL);
        download_url(url)
    }

    pub(crate) fn json_to_comic(json: JsonData) -> Comic {
        let date = parse_date(json.day, json.month, json.year);
        Comic {
            title: json.title,
            safe_title: json.safe_title,
            num: json.num,
            date,
            img_url: json.img,
            alt: json.alt,
            transcript: json.transcript,
            news: json.news,
            link: if json.link.is_empty() {
                format!("{}/{}/", BASE_URL, json.num)
            } else {
                json.link
            },
        }
    }

    fn download_url(url: String) -> Result<JsonData, XkcdError> {
        let data = attohttpc::get(url)
            .send()
            .map_err(|_| XkcdError::DownloadError)?;

        if data.is_success() {
            //TODO: fix this
            let json: JsonData = match data.json() {
                Ok(json) => json,
                Err(_) => return Err(XkcdError::JsonError),
            };
            Ok(json)
        } else {
            Err(XkcdError::DownloadError)
        }
    }
}
