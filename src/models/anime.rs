// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

use super::{
    Character, Cover, Date, Format, Link, Person, Relation, Season, Source, Status, Studio, Tag,
    Title,
};
use crate::{Client, Result};

/// Represents an anime with various attributes.
///
/// The `Anime` struct contains detailed information about an anime,
/// including its ID, title, format, status, description, dates, season,
/// episodes, duration, country of origin, licensing status, source,
/// hashtags, images, genres, synonyms, scores, popularity, tags,
/// relations, characters, staff, studios, and other metadata.
///
/// # Fields
///
/// * `id` - The ID of the anime.
/// * `id_mal` - The ID of the anime on MAL (MyAnimeList).
/// * `title` - The title of the anime.
/// * `format` - The format of the anime (e.g., TV, movie).
/// * `status` - The status of the anime (e.g., airing, completed).
/// * `description` - The description of the anime.
/// * `start_date` - The start date of the anime.
/// * `end_date` - The end date of the anime.
/// * `season` - The season of the anime.
/// * `season_year` - The year of the season of the anime.
/// * `season_int` - The integer representation of the season of the anime.
/// * `episodes` - The number of episodes of the anime.
/// * `duration` - The duration of the episodes of the anime.
/// * `country_of_origin` - The country of origin of the anime.
/// * `is_licensed` - Whether the anime is licensed or not.
/// * `source` - The source of the anime (e.g., manga, light novel).
/// * `hashtag` - The hashtag of the anime.
/// * `updated_at` - The updated date of the anime.
/// * `cover` - The cover image of the anime.
/// * `banner` - The banner image of the anime.
/// * `genres` - The genres of the anime.
/// * `synonyms` - The synonyms of the anime.
/// * `average_score` - The average score of the anime.
/// * `mean_score` - The mean score of the anime.
/// * `popularity` - The popularity of the anime.
/// * `is_locked` - Whether the anime is locked or not.
/// * `trending` - The trending of the anime.
/// * `favourites` - The number of favourites of the anime.
/// * `tags` - The tags of the anime.
/// * `relations` - The relations of the anime.
/// * `characters` - The characters of the anime.
/// * `staff` - The staff of the anime.
/// * `studios` - The studios of the anime.
/// * `is_favourite` - Whether the anime is favourite or not.
/// * `is_favourite_blocked` - Whether the anime is favourite blocked or not.
/// * `is_adult` - Whether the anime is adult or not.
/// * `next_airing_episode` - The next airing episode of the anime.
/// * `external_links` - The external links of the anime.
/// * `streaming_episodes` - The streaming episodes of the anime.
/// * `url` - The site URL of the anime.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Anime {
    /// The ID of the anime.
    pub id: i64,
    /// The ID of the anime on MAL.
    pub id_mal: Option<i64>,
    /// The title of the anime.
    pub title: Title,
    /// The format of the anime.
    pub format: Format,
    /// The status of the anime.
    pub status: Status,
    /// The description of the anime.
    pub description: String,
    /// The start date of the anime.
    pub start_date: Option<Date>,
    /// The end date of the anime.
    pub end_date: Option<Date>,
    /// The season of the anime.
    pub season: Option<Season>,
    /// The year of the season of the anime.
    pub season_year: Option<i64>,
    /// The integer representation of the season of the anime.
    pub season_int: Option<i64>,
    /// The number of episodes of the anime.
    pub episodes: Option<i64>,
    /// The duration of the episodes of the anime.
    pub duration: Option<i64>,
    /// The country of origin of the anime.
    pub country_of_origin: Option<String>,
    /// Whether the anime is licensed or not.
    pub is_licensed: Option<bool>,
    /// The source of the anime.
    pub source: Option<Source>,
    /// The hashtag of the anime.
    pub hashtag: Option<String>,
    /// The updated date of the anime.
    pub updated_at: Option<i64>,
    /// The cover image of the anime.
    #[serde(rename = "coverImage")]
    pub cover: Cover,
    /// The banner image of the anime.
    #[serde(rename = "bannerImage")]
    pub banner: Option<String>,
    /// The genres of the anime.
    pub genres: Option<Vec<String>>,
    /// The synonyms of the anime.
    pub synonyms: Option<Vec<String>>,
    /// The average score of the anime.
    pub average_score: Option<i64>,
    /// The mean score of the anime.
    pub mean_score: Option<i64>,
    /// The popularity of the anime.
    pub popularity: Option<i64>,
    /// Whether the anime is locked or not.
    pub is_locked: Option<bool>,
    /// The trending of the anime.
    pub trending: Option<i64>,
    /// The number of favourites of the anime.
    pub favourites: Option<i64>,
    /// The tags of the anime.
    pub tags: Option<Vec<Tag>>,
    /// The relations of the anime.
    #[serde(skip)]
    pub relations: Option<Vec<Relation>>,
    /// The characters of the anime.
    #[serde(skip)]
    pub characters: Option<Vec<Character>>,
    /// The staff of the anime.
    #[serde(skip)]
    pub staff: Option<Vec<Person>>,
    /// The studios of the anime.
    #[serde(skip)]
    pub studios: Option<Vec<Studio>>,
    /// Whether the anime is favourite or not.
    pub is_favourite: Option<bool>,
    /// Whether the anime is favourite blocked or not.
    pub is_favourite_blocked: Option<bool>,
    /// Whether the anime is adult or not.
    pub is_adult: Option<bool>,
    /// The next airing episode of the anime.
    pub next_airing_episode: Option<AiringSchedule>,
    /// The external links of the anime.
    pub external_links: Option<Vec<Link>>,
    /// The streaming episodes of the anime.
    pub streaming_episodes: Option<Vec<Link>>,
    /// The site URL of the anime.
    #[serde(rename = "siteUrl")]
    pub url: String,

    /// The client used to fetch additional data.
    #[serde(skip)]
    pub(crate) client: Client,
    /// Whether the person's data is fully loaded.
    #[serde(default)]
    pub(crate) is_full_loaded: bool,
}

impl Anime {
    /// Loads the full details of the anime.
    ///
    /// # Errors
    ///
    /// Returns an error if the anime details cannot be loaded.
    ///
    /// # Panics
    ///
    /// Panics if the anime is already fully loaded.
    pub async fn load_full(self) -> Result<Self> {
        if !self.is_full_loaded {
            self.client.get_anime(self.id).await
        } else {
            panic!("This anime is already full loaded!")
        }
    }
}

/// Represents the airing schedule of an anime.
///
/// The `AiringSchedule` struct contains information about the airing
/// schedule of an anime, including the ID, airing date, time until
/// airing, and the episode number.
///
/// # Fields
///
/// * `id` - The ID of the airing schedule.
/// * `at` - The airing date, represented as a Unix timestamp.
/// * `time_until` - The time until the airing, represented in seconds.
/// * `episode` - The episode number that is airing.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AiringSchedule {
    /// The ID of the airing schedule.
    id: i64,
    /// The airing date.
    #[serde(rename = "airingAt")]
    at: i64,
    /// Time until the airing.
    #[serde(rename = "timeUntilAiring")]
    time_until: i64,
    /// The airing episode.
    episode: i64,
}
