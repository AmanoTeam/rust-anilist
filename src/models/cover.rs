// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

use crate::models::Color;

/// Represents the cover images of various sizes and the color of the cover.
///
/// The `Cover` struct contains URLs for the cover images in different sizes
/// (extra large, large, and medium) and an optional color.
///
/// # Fields
///
/// * `extra_large` - The URL of the cover image in extra large size.
/// * `large` - The URL of the cover image in large size.
/// * `medium` - The URL of the cover image in medium size.
/// * `color` - The color of the cover image.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Cover {
    /// The URL of the cover image in extra large size.
    pub extra_large: Option<String>,
    /// The URL of the cover image in large size.
    pub large: Option<String>,
    /// The URL of the cover image in medium size.
    pub medium: Option<String>,
    /// The color of the cover image.
    pub color: Option<Color>,
}
