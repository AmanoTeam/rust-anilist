// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Source` enum.

use serde::{Deserialize, Serialize};

/// Represents the source of a media.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum Source {
    /// The original source.
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
    #[default]
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

impl Source {
    /// Returns a summary of the source.
    pub fn summary(&self) -> &str {
        match self {
            Source::Original => "An original production not based of another work",
            Source::Manga => "Asian comic book",
            Source::LightNovel => {
                "A written work published in volumes primarily targeting young adults and teens"
            }
            Source::VisualNovel => {
                "A video game genre that originated in Japan, featuring mostly static graphics"
            }
            Source::VideoGame => {
                "An electronic game that involves interaction with a user interface"
            }
            Source::Other => "A source that does not fit any other category",
            Source::Novel => "A written work not published in volumes",
            Source::Doujinshi => "Self-published works",
            Source::Anime => "Japanese animated productions",
            Source::WebNovel => "A novel published online",
            Source::LiveAction => {
                "A work that involves live action rather than animation such as movies or TV shows"
            }
            Source::Game => "A competitive activity or sport excluding video games",
            Source::Comic => "A publication that consists of comic art in the form of sequential panels excluding manga",
            Source::MultimediaProject => "A work that is a collection of multiple media",
            Source::PictureBook => "A book with pictures and little text",
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Source::from("original"), Source::Original);
        assert_eq!(Source::from("MANGA"), Source::Manga);
        assert_eq!(Source::from("light_novel"), Source::LightNovel);
        assert_eq!(Source::from("VISUAL_NOVEL"), Source::VisualNovel);
        assert_eq!(Source::from("video_game"), Source::VideoGame);
        assert_eq!(Source::from("other"), Source::Other);
        assert_eq!(Source::from("NOVEL"), Source::Novel);
        assert_eq!(Source::from("doujinshi"), Source::Doujinshi);
        assert_eq!(Source::from("ANIME"), Source::Anime);
        assert_eq!(Source::from("web_novel"), Source::WebNovel);
        assert_eq!(Source::from("LIVE_ACTION"), Source::LiveAction);
        assert_eq!(Source::from("game"), Source::Game);
        assert_eq!(Source::from("COMIC"), Source::Comic);
        assert_eq!(
            Source::from("multimedia_project"),
            Source::MultimediaProject
        );
        assert_eq!(Source::from("picture_book"), Source::PictureBook);
        assert_eq!(Source::from("unknown"), Source::Other); // Default case
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Source::from("original".to_string()), Source::Original);
        assert_eq!(Source::from("MANGA".to_string()), Source::Manga);
        assert_eq!(Source::from("light_novel".to_string()), Source::LightNovel);
        assert_eq!(
            Source::from("VISUAL_NOVEL".to_string()),
            Source::VisualNovel
        );
        assert_eq!(Source::from("video_game".to_string()), Source::VideoGame);
        assert_eq!(Source::from("other".to_string()), Source::Other);
        assert_eq!(Source::from("NOVEL".to_string()), Source::Novel);
        assert_eq!(Source::from("doujinshi".to_string()), Source::Doujinshi);
        assert_eq!(Source::from("ANIME".to_string()), Source::Anime);
        assert_eq!(Source::from("web_novel".to_string()), Source::WebNovel);
        assert_eq!(Source::from("LIVE_ACTION".to_string()), Source::LiveAction);
        assert_eq!(Source::from("game".to_string()), Source::Game);
        assert_eq!(Source::from("COMIC".to_string()), Source::Comic);
        assert_eq!(
            Source::from("multimedia_project".to_string()),
            Source::MultimediaProject
        );
        assert_eq!(
            Source::from("picture_book".to_string()),
            Source::PictureBook
        );
        assert_eq!(Source::from("unknown".to_string()), Source::Other); // Default case
    }
}
