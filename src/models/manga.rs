// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

use super::{
    Character, Cover, Date, Format, Link, Person, Relation, Source, Status, Studio, Tag, Title,
};
use crate::{Client, Result};

/// A manga.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Manga {
    /// The ID of the manga.
    pub id: i64,
    /// The ID of the manga on MAL.
    pub id_mal: Option<i64>,
    /// The title of the manga.
    pub title: Title,
    /// The format of the manga.
    pub format: Format,
    /// The status of the manga.
    pub status: Status,
    /// The description of the manga.
    pub description: String,
    /// The start date of the manga.
    pub start_date: Option<Date>,
    /// The end date of the manga.
    pub end_date: Option<Date>,
    /// The number of chapters of the manga.
    pub chapters: Option<i64>,
    /// The number of volumes of the manga.
    pub volumes: Option<i64>,
    /// The country of origin of the manga.
    pub country_of_origin: Option<String>,
    /// Whether the manga is licensed or not.
    pub is_licensed: Option<bool>,
    /// The source of the manga.
    pub source: Option<Source>,
    /// The hashtag of the manga.
    pub hashtag: Option<String>,
    /// The updated date of the manga.
    pub updated_at: Option<i64>,
    /// The cover image of the manga.
    #[serde(rename = "coverImage")]
    pub cover: Cover,
    /// The banner image of the manga.
    #[serde(rename = "bannerImage")]
    pub banner: Option<String>,
    /// The genres of the manga.
    pub genres: Option<Vec<String>>,
    /// The synonyms of the manga.
    pub synonyms: Option<Vec<String>>,
    /// The average score of the manga.
    pub average_score: Option<i64>,
    /// The mean score of the manga.
    pub mean_score: Option<i64>,
    /// The popularity of the manga.
    pub popularity: Option<i64>,
    /// Whether the manga is locked or not.
    pub is_locked: Option<bool>,
    /// The trending of the manga.
    pub trending: Option<i64>,
    /// The number of favourites of the manga.
    pub favourites: Option<i64>,
    /// The tags of the manga.
    pub tags: Option<Vec<Tag>>,
    /// The relations of the manga.
    #[serde(skip)]
    pub relations: Option<Vec<Relation>>,
    /// The characters of the manga.
    #[serde(skip)]
    pub characters: Option<Vec<Character>>,
    /// The staff of the manga.
    #[serde(skip)]
    pub staff: Option<Vec<Person>>,
    /// The studios of the manga.
    #[serde(skip)]
    pub studios: Option<Vec<Studio>>,
    /// Whether the manga is favourite or not.
    pub is_favourite: Option<bool>,
    /// Whether the manga is blocked or not.
    pub is_favourite_blocked: Option<bool>,
    /// Whether the manga is adult or not.
    pub is_adult: Option<bool>,
    /// The external links of the manga.
    pub external_links: Option<Vec<Link>>,
    /// The site URL of the manga.
    #[serde(rename = "siteUrl")]
    pub url: String,
    #[serde(skip)]
    pub(crate) is_full_loaded: bool,
}

impl Manga {
    /// Load fully the manga.
    ///
    /// # Errors
    ///
    /// Returns an error if the manga is already full loaded.
    pub async fn load_full(self) -> Result<Self> {
        if !self.is_full_loaded {
            let mut manga = Client::default().get_manga(self.id).await.unwrap();
            manga.is_full_loaded = true;

            Ok(manga)
        } else {
            panic!("This manga is already full loaded")
        }
    }
}
