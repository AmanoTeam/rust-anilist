// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Format` struct.

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

    /// Returns a summary of the format.
    pub fn summary(&self) -> &str {
        match self {
            Format::Tv => "Anime broadcast on television",
            Format::TvShort => "Anime which are under 15 minutes in length and broadcast on television",
            Format::Movie => "Anime movies with a theatrical release",
            Format::Special => "Special episodes that have been included in DVD/Blu-ray releases, picture dramas, pilots, etc",
            Format::Ova => "(Original Video Animation) Anime that have been released directly on DVD/Blu-ray without originally going through a theatrical release or television broadcast",
            Format::Ona => "(Original Net Animation) Anime that have been originally released online or are only available through streaming services.",
            Format::Music => "Short anime released as a music video",
            Format::Manga => "Professionally published manga with more than one chapter",
            Format::Novel => "Written books released as a series of light novels",
            Format::OneShot => "Manga with just one chapter",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_name() {
        assert_eq!(Format::Tv.name(), "TV");
        assert_eq!(Format::TvShort.name(), "TV Short");
        assert_eq!(Format::Movie.name(), "Movie");
        assert_eq!(Format::Special.name(), "Special");
        assert_eq!(Format::Ova.name(), "OVA");
        assert_eq!(Format::Ona.name(), "ONA");
        assert_eq!(Format::Music.name(), "Music");
        assert_eq!(Format::Manga.name(), "Manga");
        assert_eq!(Format::Novel.name(), "Novel");
        assert_eq!(Format::OneShot.name(), "One-Shot");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Format::from("tv"), Format::Tv);
        assert_eq!(Format::from("TV_SHORT"), Format::TvShort);
        assert_eq!(Format::from("movie"), Format::Movie);
        assert_eq!(Format::from("SPECIAL"), Format::Special);
        assert_eq!(Format::from("ova"), Format::Ova);
        assert_eq!(Format::from("ONA"), Format::Ona);
        assert_eq!(Format::from("music"), Format::Music);
        assert_eq!(Format::from("MANGA"), Format::Manga);
        assert_eq!(Format::from("novel"), Format::Novel);
        assert_eq!(Format::from("ONE_SHOT"), Format::OneShot);
        assert_eq!(Format::from("unknown"), Format::Tv); // Default case
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Format::from("tv".to_string()), Format::Tv);
        assert_eq!(Format::from("TV_SHORT".to_string()), Format::TvShort);
        assert_eq!(Format::from("movie".to_string()), Format::Movie);
        assert_eq!(Format::from("SPECIAL".to_string()), Format::Special);
        assert_eq!(Format::from("ova".to_string()), Format::Ova);
        assert_eq!(Format::from("ONA".to_string()), Format::Ona);
        assert_eq!(Format::from("music".to_string()), Format::Music);
        assert_eq!(Format::from("MANGA".to_string()), Format::Manga);
        assert_eq!(Format::from("novel".to_string()), Format::Novel);
        assert_eq!(Format::from("ONE_SHOT".to_string()), Format::OneShot);
        assert_eq!(Format::from("unknown".to_string()), Format::Tv); // Default case
    }
}
