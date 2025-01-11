// SPDX-License-Identifier: MIT↴
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>↴

//! This module contains the `Media` enum.

use serde::{Deserialize, Serialize};

use super::{Anime, Format, Manga};

/// Represents different types of media.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub enum Media {
    /// Represents an anime media type.
    Anime(Anime),
    /// Represents a manga media type.
    Manga(Manga),
    /// Represents an unknown media type.
    ///
    /// This variant is used when the media type is unknown or unsupported.
    #[default]
    Unknown,
}

impl Media {
    /// Returns the id of the media.
    pub fn id(&self) -> i64 {
        match self {
            Media::Anime(anime) => anime.id,
            Media::Manga(manga) => manga.id,
            Media::Unknown => 0,
        }
    }

    /// Returns the title of the media.
    pub fn title(&self) -> &str {
        match self {
            Media::Anime(anime) => anime.title.romaji(),
            Media::Manga(manga) => manga.title.romaji(),
            Media::Unknown => "Unknown",
        }
    }

    /// Returns the format of the media.
    pub fn format(&self) -> Option<&Format> {
        match self {
            Media::Anime(anime) => Some(&anime.format),
            Media::Manga(manga) => Some(&manga.format),
            Media::Unknown => None,
        }
    }
}

impl From<Anime> for Media {
    fn from(anime: Anime) -> Self {
        Media::Anime(anime)
    }
}

impl From<Manga> for Media {
    fn from(manga: Manga) -> Self {
        Media::Manga(manga)
    }
}
