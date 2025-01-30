// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub season_year: Option<u32>,
    /// The integer representation of the season of the anime.
    pub season_int: Option<u64>,
    /// The number of episodes of the anime.
    pub episodes: Option<u16>,
    /// The duration of the episodes of the anime.
    pub duration: Option<u8>,
    /// The country of origin of the anime.
    pub country_of_origin: Option<String>,
    /// Whether the anime is licensed or not.
    pub is_licensed: Option<bool>,
    /// The source of the anime.
    pub source: Option<Source>,
    /// The hashtag of the anime.
    pub hashtag: Option<String>,
    /// The updated date of the anime.
    pub updated_at: Option<u64>,
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
    pub average_score: Option<u8>,
    /// The mean score of the anime.
    pub mean_score: Option<u8>,
    /// The popularity of the anime.
    pub popularity: Option<u32>,
    /// Whether the anime is locked or not.
    pub is_locked: Option<bool>,
    /// The trending of the anime.
    pub trending: Option<u32>,
    /// The number of favourites of the anime.
    pub favourites: Option<u32>,
    /// The tags of the anime.
    pub tags: Option<Vec<Tag>>,
    /// The relations of the anime.
    pub(crate) relations: Value,
    /// The characters of the anime.
    pub(crate) characters: Value,
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
    pub is_adult: bool,
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
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rust_anilist::{models::Anime, Result};
    /// #
    /// # async fn f(anime: Anime) -> Result<()> {
    /// let anime = anime.load_full().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load_full(self) -> Result<Self> {
        if !self.is_full_loaded {
            self.client.get_anime(self.id).await
        } else {
            panic!("This anime is already full loaded!")
        }
    }

    /// Returns the characters of the anime.
    pub fn characters(&self) -> Result<Vec<Character>> {
        let binding = Vec::new();
        let edges = self
            .characters
            .as_object()
            .and_then(|obj| obj.get("edges"))
            .and_then(|edges| edges.as_array())
            .unwrap_or(&binding);

        let mut characters = Vec::with_capacity(edges.len());

        for edge in edges {
            let binding = serde_json::Map::new();
            let obj = edge.as_object().unwrap_or(&binding);
            let node = obj.get("node").unwrap_or(&Value::Null);
            let role = obj.get("role").and_then(|role| role.as_str()).unwrap_or("");

            let mut character: Character = serde_json::from_value(node.clone()).unwrap_or_default();
            character.role = Some(role.into());
            characters.push(character);
        }

        Ok(characters)
    }

    /// Returns the relations of the anime.
    pub fn relations(&self) -> Result<Vec<Relation>> {
        let binding = Vec::new();
        let edges = self
            .relations
            .as_object()
            .and_then(|obj| obj.get("edges"))
            .and_then(|edges| edges.as_array())
            .unwrap_or(&binding);

        let relations = edges
            .iter()
            .map(|r| serde_json::from_value(r.clone()).unwrap_or_default())
            .collect();

        Ok(relations)
    }
}

/// Represents the airing schedule of an anime.
///
/// The `AiringSchedule` struct contains information about the airing
/// schedule of an anime, including the ID, airing date, time until
/// airing, and the episode number.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AiringSchedule {
    /// The ID of the airing schedule.
    pub id: u32,
    /// The airing date.
    #[serde(rename = "airingAt")]
    pub at: i64,
    /// Time until the airing.
    #[serde(rename = "timeUntilAiring")]
    pub time_until: u64,
    /// The airing episode.
    pub episode: u32,
}
