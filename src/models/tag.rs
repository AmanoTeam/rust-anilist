// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

/// Represents a tag in the system.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Tag {
    /// The ID of the tag.
    pub id: i64,
    /// The name of the tag.
    pub name: String,
    /// The description of the tag.
    pub description: String,
    /// The category of the tag.
    pub category: String,
    /// The rank of the tag.
    pub rank: i64,
    /// Whether the tag is a general spoiler.
    pub is_general_spoiler: bool,
    /// Whether the tag is a media spoiler.
    pub is_media_spoiler: bool,
    /// Whether the tag is adult content.
    pub is_adult: bool,
    /// The user ID associated with the tag.
    pub user_id: Option<i64>,
}
