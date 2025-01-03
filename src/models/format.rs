// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

/// Represents the format of a media item.
///
/// The `Format` enum defines various formats that a media item can have,
/// such as TV shows, movies, specials, OVAs, ONAs, music, manga, novels,
/// and one-shots.
///
/// # Variants
///
/// * `Tv` - Represents a TV show.
/// * `TvShort` - Represents a short TV show.
/// * `Movie` - Represents a movie.
/// * `Special` - Represents a special.
/// * `Ova` - Represents an original video animation.
/// * `Ona` - Represents an original net animation.
/// * `Music` - Represents a music video.
/// * `Manga` - Represents a manga.
/// * `Novel` - Represents a novel.
/// * `OneShot` - Represents a one-shot.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum Format {
    /// Represents a TV show.
    #[default]
    Tv,
    /// Represents a short TV show.
    TvShort,
    /// Represents a movie.
    Movie,
    /// Represents a special.
    Special,
    /// Represents an original video animation.
    Ova,
    /// Represents an original net animation.
    Ona,
    /// Represents a music video.
    Music,
    /// Represents a manga.
    Manga,
    /// Represents a novel.
    Novel,
    /// Represents a one-shot.
    OneShot,
}

impl Format {
    /// Returns the name of the format.
    pub fn name(&self) -> &str {
        match self {
            Format::Tv => "TV",
            Format::TvShort => "TV Short",
            Format::Movie => "Movie",
            Format::Special => "Special",
            Format::Ova => "OVA",
            Format::Ona => "ONA",
            Format::Music => "Music",
            Format::Manga => "Manga",
            Format::Novel => "Novel",
            Format::OneShot => "One-Shot",
        }
    }
}

impl From<&str> for Format {
    fn from(value: &str) -> Self {
        match value.trim().to_uppercase().as_str() {
            "TV" => Format::Tv,
            "TV_SHORT" => Format::TvShort,
            "MOVIE" => Format::Movie,
            "SPECIAL" => Format::Special,
            "OVA" => Format::Ova,
            "ONA" => Format::Ona,
            "MUSIC" => Format::Music,
            "MANGA" => Format::Manga,
            "NOVEL" => Format::Novel,
            "ONE_SHOT" => Format::OneShot,
            _ => Format::default(),
        }
    }
}

impl From<String> for Format {
    fn from(value: String) -> Self {
        Format::from(value.as_str())
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
