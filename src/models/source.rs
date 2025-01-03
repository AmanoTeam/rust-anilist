// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

/// Represents the source of a media.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum Source {
    /// The original source.
    #[default]
    Original,
    /// Manga source.
    Manga,
    /// Light novel source.
    LightNovel,
    /// Visual novel source.
    VisualNovel,
    /// Video game source.
    VideoGame,
    /// Other source.
    Other,
    /// Novel source.
    Novel,
    /// Doujinshi source.
    Doujinshi,
    /// Anime source.
    Anime,
    /// Web novel source.
    WebNovel,
    /// Live action source.
    LiveAction,
    /// Game source.
    Game,
    /// Comic source.
    Comic,
    /// Multimedia project source.
    MultimediaProject,
    /// Picture book source.
    PictureBook,
}

impl From<&str> for Source {
    fn from(source: &str) -> Self {
        match source.to_ascii_uppercase().as_str() {
            "ORIGINAL" => Source::Original,
            "MANGA" => Source::Manga,
            "LIGHT_NOVEL" => Source::LightNovel,
            "VISUAL_NOVEL" => Source::VisualNovel,
            "VIDEO_GAME" => Source::VideoGame,
            "OTHER" => Source::Other,
            "NOVEL" => Source::Novel,
            "DOUJINSHI" => Source::Doujinshi,
            "ANIME" => Source::Anime,
            "WEB_NOVEL" => Source::WebNovel,
            "LIVE_ACTION" => Source::LiveAction,
            "GAME" => Source::Game,
            "COMIC" => Source::Comic,
            "MULTIMEDIA_PROJECT" => Source::MultimediaProject,
            "PICTURE_BOOK" => Source::PictureBook,
            _ => Source::Other,
        }
    }
}

impl From<String> for Source {
    fn from(source: String) -> Self {
        Source::from(source.as_str())
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Original => write!(f, "Original"),
            Source::Manga => write!(f, "Manga"),
            Source::LightNovel => write!(f, "Light Novel"),
            Source::VisualNovel => write!(f, "Visual Novel"),
            Source::VideoGame => write!(f, "Video Game"),
            Source::Other => write!(f, "Other"),
            Source::Novel => write!(f, "Novel"),
            Source::Doujinshi => write!(f, "Doujinshi"),
            Source::Anime => write!(f, "Anime"),
            Source::WebNovel => write!(f, "Web Novel"),
            Source::LiveAction => write!(f, "Live Action"),
            Source::Game => write!(f, "Game"),
            Source::Comic => write!(f, "Comic"),
            Source::MultimediaProject => write!(f, "Multimedia Project"),
            Source::PictureBook => write!(f, "Picture Book"),
        }
    }
}
