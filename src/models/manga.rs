// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

use super::{
    Character, Cover, Date, Format, Link, Person, Relation, Source, Status, Studio, Tag, Title,
};
use crate::{Client, Result};

/// Represents a manga with various attributes.
///
/// The `Manga` struct contains detailed information about a manga,
/// including its ID, title, format, status, description, dates,
/// chapters, volumes, country of origin, licensing status, source,
/// hashtags, images, genres, synonyms, scores, popularity, tags,
/// relations, characters, staff, studios, and other metadata.
///
/// # Fields
///
/// * `id` - The ID of the manga.
/// * `id_mal` - The ID of the manga on MAL (MyAnimeList).
/// * `title` - The title of the manga.
/// * `format` - The format of the manga (e.g., manga, novel).
/// * `status` - The status of the manga (e.g., publishing, completed).
/// * `description` - The description of the manga.
/// * `start_date` - The start date of the manga.
/// * `end_date` - The end date of the manga.
/// * `chapters` - The number of chapters of the manga.
/// * `volumes` - The number of volumes of the manga.
/// * `country_of_origin` - The country of origin of the manga.
/// * `is_licensed` - Whether the manga is licensed or not.
/// * `source` - The source of the manga (e.g., original, adaptation).
/// * `hashtag` - The hashtag of the manga.
/// * `updated_at` - The updated date of the manga.
/// * `cover` - The cover image of the manga.
/// * `banner` - The banner image of the manga.
/// * `genres` - The genres of the manga.
/// * `synonyms` - The synonyms of the manga.
/// * `average_score` - The average score of the manga.
/// * `mean_score` - The mean score of the manga.
/// * `popularity` - The popularity of the manga.
/// * `is_locked` - Whether the manga is locked or not.
/// * `trending` - The trending of the manga.
/// * `favourites` - The number of favourites of the manga.
/// * `tags` - The tags of the manga.
/// * `relations` - The relations of the manga.
/// * `characters` - The characters of the manga.
/// * `staff` - The staff of the manga.
/// * `studios` - The studios of the manga.
/// * `is_favourite` - Whether the manga is favourite or not.
/// * `is_favourite_blocked` - Whether the manga is favourite blocked or not.
/// * `is_adult` - Whether the manga is adult or not.
/// * `external_links` - The external links of the manga.
/// * `url` - The site URL of the manga.
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

    /// The client used to fetch additional data.
    #[serde(skip)]
    pub(crate) client: Client,
    /// Whether the person's data is fully loaded.
    #[serde(default)]
    pub(crate) is_full_loaded: bool,
}

impl Manga {
    /// Loads the full details of the manga.
    ///
    /// # Errors
    ///
    /// Returns an error if the manga details cannot be loaded.
    ///
    /// # Panics
    ///
    /// Panics if the manga is already fully loaded.
    pub async fn load_full(self) -> Result<Self> {
        if !self.is_full_loaded {
            self.client.get_manga(self.id).await
        } else {
            panic!("This manga is already full loaded")
        }
    }
}
