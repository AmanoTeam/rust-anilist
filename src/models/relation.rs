// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Relation` struct and its related types.

use serde::{Deserialize, Serialize};

use super::{Anime, Manga, MediaType};

/// Represents a relation between different media types.
///
/// The `Relation` struct contains information about the relationship
/// between different media types, such as anime and manga, including
/// the media type, related anime or manga, relation ID, relation type,
/// and whether it is the main studio.
///
/// # Fields
///
/// * `media_type` - The type of media (e.g., anime, manga).
/// * `anime` - An optional related anime.
/// * `manga` - An optional related manga.
/// * `id` - The ID of the relation.
/// * `relation_type` - The type of relation (e.g., adaptation, sequel).
/// * `is_main_studio` - Whether the relation is the main studio.
// TODO: Use generic type
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Relation {
    /// The type of media (e.g., anime, manga).
    pub media_type: MediaType,
    /// An optional related anime.
    pub anime: Option<Anime>,
    /// An optional related manga.
    pub manga: Option<Manga>,
    /// The ID of the relation.
    pub id: i64,
    /// The type of relation (e.g., adaptation, sequel).
    pub relation_type: RelationType,
    /// Whether the relation is the main studio.
    pub is_main_studio: bool,
}

/// Represents the type of relation between different media.
///
/// The `RelationType` enum defines various types of relationships that
/// can exist between different media, such as adaptations, sequels,
/// prequels, and more.
///
/// # Variants
///
/// * `Adaptation` - The media is an adaptation of another work.
/// * `Prequel` - The media is a prequel to another work.
/// * `Sequel` - The media is a sequel to another work.
/// * `Parent` - The media is a parent story to another work.
/// * `SideStory` - The media is a side story to another work.
/// * `Character` - The media shares characters with another work.
/// * `Summary` - The media is a summary of another work.
/// * `Alternative` - The media is an alternative version of another work.
/// * `SpinOff` - The media is a spin-off of another work.
/// * `Other` - The media has some other type of relation to another work.
/// * `Source` - The media is the source material for another work.
/// * `Compilation` - The media is a compilation of another work.
/// * `Contains` - The media contains another work.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub enum RelationType {
    /// The media is an adaptation of another work.
    Adaptation,
    /// The media is a prequel to another work.
    Prequel,
    /// The media is a sequel to another work.
    Sequel,
    /// The media is a parent story to another work.
    Parent,
    /// The media is a side story to another work.
    SideStory,
    /// The media shares characters with another work.
    Character,
    /// The media is a summary of another work.
    Summary,
    /// The media is an alternative version of another work.
    Alternative,
    /// The media is a spin-off of another work.
    SpinOff,
    /// The media has some other type of relation to another work.
    #[default]
    Other,
    /// The media is the source material for another work.
    Source,
    /// The media is a compilation of another work.
    Compilation,
    /// The media contains another work.
    Contains,
}

impl RelationType {
    /// Returns a summary of the relation type.
    pub fn summary(&self) -> &str {
        match self {
            RelationType::Adaptation => "An adaption of this media into a different format",
            RelationType::Prequel => "Released before the relation",
            RelationType::Sequel => "Released after the relation",
            RelationType::Parent => "The media a side story is from",
            RelationType::SideStory => "A side story of the parent media",
            RelationType::Character => "Shares at least 1 character",
            RelationType::Summary => "A shortened and summarized version",
            RelationType::Alternative => "An alternative version of the same media",
            RelationType::SpinOff => {
                "An alternative version of the media with a different primary focus"
            }
            RelationType::Other => "Other",
            RelationType::Source => "The source material the media was adapted from",
            RelationType::Compilation => "A compilation of the media",
            RelationType::Contains => "A media that contains the relation",
        }
    }
}
