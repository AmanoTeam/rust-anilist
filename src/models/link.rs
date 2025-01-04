// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Link` struct.

use serde::{Deserialize, Serialize};

use super::{Color, Language};

/// Represents a link.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Link {
    /// The ID of the link.
    pub id: Option<i64>,
    /// The title of the link.
    pub title: Option<String>,
    /// The thumbnail of the link.
    pub thumbnail: Option<String>,
    /// The URL of the link.
    pub url: Option<String>,
    /// The site of the link.
    pub site: Option<String>,
    /// The ID of the site of the link.
    pub site_id: Option<i64>,
    /// The type of the link.
    pub link_type: Option<LinkType>,
    /// The language of the link.
    pub language: Option<Language>,
    /// The color of the link.
    pub color: Option<Color>,
    /// The icon of the link.
    pub icon: Option<String>,
    /// The notes of the link.
    pub notes: Option<String>,
    /// Whether the link is disabled or not.
    pub is_disabled: Option<bool>,
}

/// Represents the type of link.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub enum LinkType {
    /// The info link type.
    #[default]
    Info,
    /// The streaming link type.
    Streaming,
    /// The social link type.
    Social,
}

impl std::fmt::Display for LinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkType::Info => write!(f, "Info"),
            LinkType::Streaming => write!(f, "Streaming"),
            LinkType::Social => write!(f, "Social"),
        }
    }
}
